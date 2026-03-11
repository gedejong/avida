/*
 *  data/Provider.cc
 *  avida-core
 *
 *  Created by David on 10/10/11.
 *  Copyright 2011 Michigan State University. All rights reserved.
 *  http://avida.devosoft.org/
 *
 *
 *  This file is part of Avida.
 *
 *  Avida is free software; you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
 *  as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 *  Avida is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public License along with Avida.
 *  If not, see <http://www.gnu.org/licenses/>.
 *
 *  Authors: David M. Bryson <david@programerror.com>
 *
 */

#include "avida/data/Provider.h"

#include "avida/data/Package.h"
#include "avida/data/Util.h"
#include "rust/running_stats_ffi.h"

namespace {
enum DataIDKind {
  DATA_ID_INVALID = 0,
  DATA_ID_STANDARD = 1,
  DATA_ID_ARGUMENTED = 2
};
}

bool Avida::Data::Provider::SupportsConcurrentUpdate() const
{
  return false;
}


Avida::Data::PackagePtr Avida::Data::ArgumentedProvider::GetProvidedValuesForArguments(const DataID& data_id,
                                                                                       ConstArgumentSetPtr args) const
{
  Apto::SmartPtr<ArrayPackage, Apto::InternalRCObject> package(new ArrayPackage);
  for (ArgumentSet::ConstIterator it = args->Begin(); it.Next();) {
    PackagePtr comp = this->GetProvidedValueForArgument(data_id, *it.Get());
    if (!comp) continue;
    package->AddComponent(comp);
  }
  return package;
}

Avida::Data::PackagePtr Avida::Data::ArgumentedProvider::GetProvidedValue(const DataID& data_id) const
{
  PackagePtr pkg;
  char* raw_id = NULL;
  char* arg = NULL;
  int id_kind = avd_provider_classify_id((const char*) data_id, &raw_id, &arg);
  if (id_kind == DATA_ID_ARGUMENTED) {
    DataID parsed_raw((raw_id) ? raw_id : "");
    Argument argument = (arg) ? arg : "";
    avd_provider_string_free(raw_id);
    avd_provider_string_free(arg);
    return GetProvidedValueForArgument(parsed_raw, argument);
  }
  avd_provider_string_free(raw_id);
  avd_provider_string_free(arg);
  if (id_kind == DATA_ID_STANDARD) {
    return GetProvidedValueForArgument(data_id, "");
  }
  return pkg;
}
