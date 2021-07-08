# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

import timeit
import io

import pyarrow.parquet


def bench(log2_size: int, datatype: str):
    with open(f"fixtures/pyarrow3/v1/benches_{2**log2_size}.parquet", "rb") as f:
        data = f.read()
    data = io.BytesIO(data)

    def f():
        pyarrow.parquet.read_table(data, columns=[datatype])

    seconds = timeit.Timer(f).timeit(number=512) / 512
    microseconds = seconds * 1000 * 1000
    print(f"read {datatype} 2^{log2_size}     time: {microseconds:.2f} us")


# for i in range(10, 22, 2):
#    bench(i, "int64")

for i in range(10, 22, 2):
    bench(i, "string")

for i in range(10, 22, 2):
    bench(i, "bool")
