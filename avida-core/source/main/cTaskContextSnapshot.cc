// C++ helper: populate a Rust TaskContextSnapshot from cTaskContext.
//
// Called by C++ task dispatch code before delegating to Rust evaluators.
// Copies flat scalar data and the first TASK_CTX_BUFFER_CAP values from
// input/output buffers into the snapshot struct.
//
// NOTE: Task-entry arguments (task_arg_int, task_arg_double) are NOT filled
// here because cArgContainer doesn't expose array sizes and task schemas
// vary.  Callers should fill specific arg slots after calling this function.

#include "cTaskContext.h"
#include "cTaskEntry.h"
#include "cArgContainer.h"
#include "cOrganism.h"
#include "cPhenotype.h"
#include "rust/running_stats_ffi.h"

#include <algorithm>
#include <cstring>

void avd_populate_task_context(TaskContextSnapshot* snap, cTaskContext& ctx)
{
  // Zero-init the snapshot so any unused fields are deterministic.
  std::memset(snap, 0, sizeof(*snap));

  // -- cTaskContext scalars --
  snap->logic_id = ctx.GetLogicId();

  // -- Output buffer --
  const tBuffer<int>& out = ctx.GetOutputBuffer();
  int out_stored = out.GetNumStored();
  int out_cap = std::min(out_stored, TASK_CTX_BUFFER_CAP);
  snap->output_count = out_cap;
  for (int i = 0; i < out_cap; ++i) {
    snap->output_buffer[i] = out[i];
  }
  snap->output_value = (out_stored > 0) ? out[0] : 0;

  // -- Input buffer --
  const tBuffer<int>& in = ctx.GetInputBuffer();
  int in_stored = in.GetNumStored();
  int in_cap = std::min(in_stored, TASK_CTX_BUFFER_CAP);
  snap->input_count = in_cap;
  for (int i = 0; i < in_cap; ++i) {
    snap->input_buffer[i] = in[i];
  }

  // -- Task entry arguments --
  // Mark whether args exist; actual values must be filled by the caller
  // since cArgContainer doesn't expose its array sizes.
  cTaskEntry* entry = ctx.GetTaskEntry();
  snap->has_task_args = (entry && entry->HasArguments()) ? 1 : 0;

  // -- Organism fields --
  cOrganism* org = ctx.GetOrganism();
  if (org) {
    snap->cell_id            = org->GetCellID();
    snap->av_cell_id         = org->GetAVCellID();
    snap->forage_target      = org->GetForageTarget();
    snap->gradient_movement  = org->GetGradientMovement();
    snap->has_opinion        = org->HasOpinion() ? 1 : 0;
    if (snap->has_opinion) {
      snap->opinion_value    = org->GetOpinion().first;
    }
    snap->kaboom_executed    = org->GetPhenotype().GetKaboomExecuted() ? 1 : 0;
    snap->kaboom_executed2   = org->GetPhenotype().GetKaboomExecuted2() ? 1 : 0;
    snap->event_killed       = org->GetEventKilled() ? 1 : 0;
    snap->prev_seen_cell_id  = org->GetPrevSeenCellID();
  } else {
    snap->cell_id           = -1;
    snap->av_cell_id        = -1;
    snap->prev_seen_cell_id = -1;
  }

  // -- Neighbor info --
  const tList<tBuffer<int> >& neighbor_inputs = ctx.GetNeighborhoodInputBuffers();
  snap->num_neighbors_with_outputs = neighbor_inputs.GetSize();
}
