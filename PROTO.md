Trying to start server...
==== Sending event notification ====
```http
POST /event_notification HTTP/1.1
Accept: application/json
Accept-Encoding: gzip, deflate
Connection: keep-alive
Content-Length: 1034
Host: 127.0.0.1:53650
User-Agent: HTTPie/0.9.2
X-Ycm-Hmac: c5qC+pW0SkE2UPrnQKKhii/vQ9QtzZ1JyLdcOroI24g=
content-type: application/json

{
    "column_num": 1,
    "event_name": "FileReadyToParse",
    "file_data": {
        "/Users/jwilm/code/ycmd/examples/samples/some_javascript.js": {
            "contents": "// Copyright (C) 2014  Google Inc.\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//     http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\n(function() {\n  var x = 10;\n  var y = 15;\n  var foobar = x + y;\n  var foozoo = x + y;\n  // location after second 'o' is line 24, column 6\n  foo\n});\n\n",
            "filetypes": [
                "javascript"
            ]
        }
    },
    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_javascript.js",
    "line_num": 1
}

HTTP/1.1 200 OK
Content-Length: 2
Content-Type: application/json
Date: Wed, 04 Nov 2015 18:09:37 GMT
Server: waitress
X-Ycm-Hmac: eTfQflyBjjiy2gvFLYHARYRoGcteW7Sn+opGvD40H/w=

{}
```

==== Sending code-completion request ====
```http
POST /completions HTTP/1.1
Accept: application/json
Accept-Encoding: gzip, deflate
Connection: keep-alive
Content-Length: 1001
Host: 127.0.0.1:53650
User-Agent: HTTPie/0.9.2
X-Ycm-Hmac: tRVyZ20yG7+Xdlf10L8p9/4709+UBnjlwMj/dw+ZOTY=
content-type: application/json

{
    "column_num": 6,
    "file_data": {
        "/Users/jwilm/code/ycmd/examples/samples/some_javascript.js": {
            "contents": "// Copyright (C) 2014  Google Inc.\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//     http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\n(function() {\n  var x = 10;\n  var y = 15;\n  var foobar = x + y;\n  var foozoo = x + y;\n  // location after second 'o' is line 24, column 6\n  foo\n});\n\n",
            "filetypes": [
                "javascript"
            ]
        }
    },
    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_javascript.js",
    "line_num": 21
}

HTTP/1.1 200 OK
Content-Length: 229
Content-Type: application/json
Date: Wed, 04 Nov 2015 18:09:38 GMT
Server: waitress
X-Ycm-Hmac: VixOnVGyVxCbnZ4Od878fGJ417p/uKihMCPDbDrPau4=

{
    "completion_start_column": 3,
    "completions": [
        {
            "extra_menu_info": "[ID]",
            "insertion_text": "foo"
        },
        {
            "extra_menu_info": "[ID]",
            "insertion_text": "foobar"
        },
        {
            "extra_menu_info": "[ID]",
            "insertion_text": "foozoo"
        }
    ],
    "errors": []
}
```

==== Sending event notification ====
```http
POST /event_notification HTTP/1.1
Accept: application/json
Accept-Encoding: gzip, deflate
Connection: keep-alive
Content-Length: 1084
Host: 127.0.0.1:53650
User-Agent: HTTPie/0.9.2
X-Ycm-Hmac: 4pWmOOb3Oi33hQzzJHf2bM9tIok0Sfu+bisIG91lB/w=
content-type: application/json

{
    "column_num": 1,
    "event_name": "FileReadyToParse",
    "file_data": {
        "/Users/jwilm/code/ycmd/examples/samples/some_python.py": {
            "contents": "#!/usr/bin/env python\n#\n# Copyright (C) 2014  Google Inc.\n#\n# Licensed under the Apache License, Version 2.0 (the \"License\");\n# you may not use this file except in compliance with the License.\n# You may obtain a copy of the License at\n#\n#     http://www.apache.org/licenses/LICENSE-2.0\n#\n# Unless required by applicable law or agreed to in writing, software\n# distributed under the License is distributed on an \"AS IS\" BASIS,\n# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n# See the License for the specific language governing permissions and\n# limitations under the License.\n\nclass Example( object ):\n  def __init__( self ):\n    self.x = 1\n    self.y = 2\n    self.z = 3\n\n\nif __name__ == \"__main__\":\n  ex = Example()\n  # location after the dot is line 29, column 6\n  ex.\n",
            "filetypes": [
                "python"
            ]
        }
    },
    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_python.py",
    "line_num": 1
}

HTTP/1.1 200 OK
Content-Length: 2
Content-Type: application/json
Date: Wed, 04 Nov 2015 18:09:38 GMT
Server: waitress
X-Ycm-Hmac: eTfQflyBjjiy2gvFLYHARYRoGcteW7Sn+opGvD40H/w=

{}
```

==== Sending code-completion request ====
```http
POST /completions HTTP/1.1
Accept: application/json
Accept-Encoding: gzip, deflate
Connection: keep-alive
Content-Length: 1051
Host: 127.0.0.1:53650
User-Agent: HTTPie/0.9.2
X-Ycm-Hmac: UwHxWCZ5qs7zD+9/Xth+FZIVTcfKncnQWZ6gtSshgGM=
content-type: application/json

{
    "column_num": 6,
    "file_data": {
        "/Users/jwilm/code/ycmd/examples/samples/some_python.py": {
            "contents": "#!/usr/bin/env python\n#\n# Copyright (C) 2014  Google Inc.\n#\n# Licensed under the Apache License, Version 2.0 (the \"License\");\n# you may not use this file except in compliance with the License.\n# You may obtain a copy of the License at\n#\n#     http://www.apache.org/licenses/LICENSE-2.0\n#\n# Unless required by applicable law or agreed to in writing, software\n# distributed under the License is distributed on an \"AS IS\" BASIS,\n# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n# See the License for the specific language governing permissions and\n# limitations under the License.\n\nclass Example( object ):\n  def __init__( self ):\n    self.x = 1\n    self.y = 2\n    self.z = 3\n\n\nif __name__ == \"__main__\":\n  ex = Example()\n  # location after the dot is line 29, column 6\n  ex.\n",
            "filetypes": [
                "python"
            ]
        }
    },
    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_python.py",
    "line_num": 27
}

HTTP/1.1 200 OK
Content-Length: 3388
Content-Type: application/json
Date: Wed, 04 Nov 2015 18:09:38 GMT
Server: waitress
X-Ycm-Hmac: qts+xHFVsD1gNS/wVneLchN7+XAoCKnQw2344sqF+7M=

{
    "completion_start_column": 6,
    "completions": [
        {
            "extra_data": {
                "location": {
                    "column_num": 10,
                    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_python.py",
                    "line_num": 19
                }
            },
            "extra_menu_info": "statement:     self.x = 1",
            "insertion_text": "x"
        },
        {
            "extra_data": {
                "location": {
                    "column_num": 10,
                    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_python.py",
                    "line_num": 20
                }
            },
            "extra_menu_info": "statement:     self.y = 2",
            "insertion_text": "y"
        },
        {
            "extra_data": {
                "location": {
                    "column_num": 10,
                    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_python.py",
                    "line_num": 21
                }
            },
            "extra_menu_info": "statement:     self.z = 3",
            "insertion_text": "z"
        },
        {
            "detailed_info": "type(object) -> the object's type\ntype(name, bases, dict) -> a new type",
            "extra_menu_info": "class: __builtin__.object.type",
            "insertion_text": "__class__"
        },
        {
            "detailed_info": "x.__delattr__('name') <==> del x.name",
            "extra_menu_info": "function: __builtin__.object.__delattr__",
            "insertion_text": "__delattr__"
        },
        {
            "detailed_info": "str(object='') -> string\n\nReturn a nice string representation of the object.\nIf the argument is a string, the return value is the same object.",
            "extra_menu_info": "instance: __builtin__.object.str",
            "insertion_text": "__doc__"
        },
        {
            "detailed_info": "default object formatter",
            "extra_menu_info": "function: __builtin__.object.__format__",
            "insertion_text": "__format__"
        },
        {
            "detailed_info": "x.__getattribute__('name') <==> x.name",
            "extra_menu_info": "function: __builtin__.object.__getattribute__",
            "insertion_text": "__getattribute__"
        },
        {
            "detailed_info": "x.__hash__() <==> hash(x)",
            "extra_menu_info": "function: __builtin__.object.__hash__",
            "insertion_text": "__hash__"
        },
        {
            "detailed_info": "x.__init__(...) initializes x; see help(type(x)) for signature",
            "extra_menu_info": "function: __builtin__.object.__init__",
            "insertion_text": "__init__"
        },
        {
            "detailed_info": "T.__new__(S, ...) -> a new object with type S, a subtype of T",
            "extra_menu_info": "function: __builtin__.object.__new__",
            "insertion_text": "__new__"
        },
        {
            "detailed_info": "helper for pickle",
            "extra_menu_info": "function: __builtin__.object.__reduce__",
            "insertion_text": "__reduce__"
        },
        {
            "detailed_info": "helper for pickle",
            "extra_menu_info": "function: __builtin__.object.__reduce_ex__",
            "insertion_text": "__reduce_ex__"
        },
        {
            "detailed_info": "x.__repr__() <==> repr(x)",
            "extra_menu_info": "function: __builtin__.object.__repr__",
            "insertion_text": "__repr__"
        },
        {
            "detailed_info": "x.__setattr__('name', value) <==> x.name = value",
            "extra_menu_info": "function: __builtin__.object.__setattr__",
            "insertion_text": "__setattr__"
        },
        {
            "detailed_info": "__sizeof__() -> int\nsize of object in memory, in bytes",
            "extra_menu_info": "function: __builtin__.object.__sizeof__",
            "insertion_text": "__sizeof__"
        },
        {
            "detailed_info": "x.__str__() <==> str(x)",
            "extra_menu_info": "function: __builtin__.object.__str__",
            "insertion_text": "__str__"
        },
        {
            "detailed_info": "Abstract classes can override this to customize issubclass().\n\nThis is invoked early on by abc.ABCMeta.__subclasscheck__().\nIt should return True, False or NotImplemented.  If it returns\nNotImplemented, the normal algorithm is used.  Otherwise, it\noverrides the normal algorithm (and the outcome is cached).",
            "extra_menu_info": "function: __builtin__.object.__subclasshook__",
            "insertion_text": "__subclasshook__"
        }
    ],
    "errors": []
}
```

```http
POST /load_extra_conf_file HTTP/1.1
Accept: application/json
Accept-Encoding: gzip, deflate
Connection: keep-alive
Content-Length: 66
Host: 127.0.0.1:53650
User-Agent: HTTPie/0.9.2
X-Ycm-Hmac: zfChhM6c9u/lTzfOleSqV0aOyXI/BOAKqftT8Colivo=
content-type: application/json

{
    "filepath": "/Users/jwilm/code/ycmd/examples/.ycm_extra_conf.py"
}

HTTP/1.1 200 OK
Content-Length: 0
Content-Type: text/html; charset=UTF-8
Date: Wed, 04 Nov 2015 18:09:39 GMT
Server: waitress
X-Ycm-Hmac: yUqqj6d/XHjn2UbjzyP1JHe4yLnjhW5xiOgLGOg+Fxc=


```

==== Sending event notification ====
```http
POST /event_notification HTTP/1.1
Accept: application/json
Accept-Encoding: gzip, deflate
Connection: keep-alive
Content-Length: 1033
Host: 127.0.0.1:53650
User-Agent: HTTPie/0.9.2
X-Ycm-Hmac: 4uB+HgiImsyz37m2/fhxOthu03amksumHtUDZp2vfjA=
content-type: application/json

{
    "column_num": 1,
    "event_name": "FileReadyToParse",
    "file_data": {
        "/Users/jwilm/code/ycmd/examples/samples/some_cpp.cpp": {
            "contents": "// Copyright (C) 2014  Google Inc.\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//     http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nstruct Foo {\n  int x;\n  int y  // There's a missing semicolon here\n  char c;\n};\n\nint main()\n{\n  Foo foo;\n  // The location after the dot is line 25, col 7\n  foo.\n}\n",
            "filetypes": [
                "cpp"
            ]
        }
    },
    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_cpp.cpp",
    "line_num": 1
}

HTTP/1.1 200 OK
Content-Length: 813
Content-Type: application/json
Date: Wed, 04 Nov 2015 18:09:39 GMT
Server: waitress
X-Ycm-Hmac: 41I0XmxcOGhhIKlaHrJQkSiVE1/4WbAuRRj0b8/Qo1M=

[
    {
        "fixit_available": true,
        "kind": "ERROR",
        "location": {
            "column_num": 8,
            "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_cpp.cpp",
            "line_num": 17
        },
        "location_extent": {
            "end": {
                "column_num": 0,
                "filepath": "",
                "line_num": 0
            },
            "start": {
                "column_num": 0,
                "filepath": "",
                "line_num": 0
            }
        },
        "ranges": [],
        "text": "expected ';' at end of declaration list"
    },
    {
        "fixit_available": false,
        "kind": "ERROR",
        "location": {
            "column_num": 1,
            "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_cpp.cpp",
            "line_num": 26
        },
        "location_extent": {
            "end": {
                "column_num": 2,
                "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_cpp.cpp",
                "line_num": 26
            },
            "start": {
                "column_num": 1,
                "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_cpp.cpp",
                "line_num": 26
            }
        },
        "ranges": [],
        "text": "expected unqualified-id"
    }
]
```

==== Sending code-completion request ====
```http
POST /completions HTTP/1.1
Accept: application/json
Accept-Encoding: gzip, deflate
Connection: keep-alive
Content-Length: 1000
Host: 127.0.0.1:53650
User-Agent: HTTPie/0.9.2
X-Ycm-Hmac: AvgGe+4Su+oonYn3HyQFK6941LT5xzeBCZQh+6KSIPc=
content-type: application/json

{
    "column_num": 7,
    "file_data": {
        "/Users/jwilm/code/ycmd/examples/samples/some_cpp.cpp": {
            "contents": "// Copyright (C) 2014  Google Inc.\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//     http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nstruct Foo {\n  int x;\n  int y  // There's a missing semicolon here\n  char c;\n};\n\nint main()\n{\n  Foo foo;\n  // The location after the dot is line 25, col 7\n  foo.\n}\n",
            "filetypes": [
                "cpp"
            ]
        }
    },
    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_cpp.cpp",
    "line_num": 25
}

HTTP/1.1 200 OK
Content-Length: 723
Content-Type: application/json
Date: Wed, 04 Nov 2015 18:09:39 GMT
Server: waitress
X-Ycm-Hmac: ESyeWPxgZs8Lrrp9wrm/IkFsZ6qujs3LM8M+AlQvPsE=

{
    "completion_start_column": 7,
    "completions": [
        {
            "detailed_info": "int y\n",
            "extra_menu_info": "int",
            "insertion_text": "y",
            "kind": "MEMBER",
            "menu_text": "y"
        },
        {
            "detailed_info": "int x\n",
            "extra_menu_info": "int",
            "insertion_text": "x",
            "kind": "MEMBER",
            "menu_text": "x"
        },
        {
            "detailed_info": " Foo::\n",
            "insertion_text": "Foo::",
            "kind": "STRUCT",
            "menu_text": "Foo::"
        },
        {
            "detailed_info": "Foo & operator=( const Foo & )\nFoo & operator=( Foo && )\n",
            "extra_menu_info": "Foo &",
            "insertion_text": "operator=",
            "kind": "FUNCTION",
            "menu_text": "operator=( const Foo & )"
        },
        {
            "detailed_info": "void ~Foo()\n",
            "extra_menu_info": "void",
            "insertion_text": "~Foo",
            "kind": "FUNCTION",
            "menu_text": "~Foo()"
        }
    ],
    "errors": []
}
```

==== Sending event notification ====
```http
POST /event_notification HTTP/1.1
Accept: application/json
Accept-Encoding: gzip, deflate
Connection: keep-alive
Content-Length: 455
Host: 127.0.0.1:53650
User-Agent: HTTPie/0.9.2
X-Ycm-Hmac: Ic6rtNbgj89GsbsqjYWc5hIHkl1yVSh0CG06cD9QnvM=
content-type: application/json

{
    "column_num": 1,
    "event_name": "FileReadyToParse",
    "file_data": {
        "/Users/jwilm/code/ycmd/examples/samples/some_csharp.cs": {
            "contents": "using System;\n\nnamespace some_csharp\n{\n  class MainClass\n  {\n    public static void Main (string[] args)\n    {\n      // location after second dot is line 9, column 15\n      Console.\n    }\n  }\n}\n",
            "filetypes": [
                "cs"
            ]
        }
    },
    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_csharp.cs",
    "line_num": 1
}

HTTP/1.1 200 OK
Content-Length: 2
Content-Type: application/json
Date: Wed, 04 Nov 2015 18:09:40 GMT
Server: waitress
X-Ycm-Hmac: eTfQflyBjjiy2gvFLYHARYRoGcteW7Sn+opGvD40H/w=

{}
```

Waiting for OmniSharpServer to become ready...
==== Sending code-completion request ====
```http
POST /completions HTTP/1.1
Accept: application/json
Accept-Encoding: gzip, deflate
Connection: keep-alive
Content-Length: 423
Host: 127.0.0.1:53650
User-Agent: HTTPie/0.9.2
X-Ycm-Hmac: hO2CL1nD71B6kzBmmP+AShiNh8yDTTQ43X7plBwhfho=
content-type: application/json

{
    "column_num": 15,
    "file_data": {
        "/Users/jwilm/code/ycmd/examples/samples/some_csharp.cs": {
            "contents": "using System;\n\nnamespace some_csharp\n{\n  class MainClass\n  {\n    public static void Main (string[] args)\n    {\n      // location after second dot is line 9, column 15\n      Console.\n    }\n  }\n}\n",
            "filetypes": [
                "cs"
            ]
        }
    },
    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_csharp.cs",
    "line_num": 10
}

HTTP/1.1 200 OK
Content-Length: 18239
Content-Type: application/json
Date: Wed, 04 Nov 2015 18:09:43 GMT
Server: waitress
X-Ycm-Hmac: qkwzdOhVVol3AZIdXUWcW35idkXPLUGSHH2g8SnYF6o=

{
    "completion_start_column": 15,
    "completions": [
        {
            "detailed_info": "ConsoleColor BackgroundColor { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "ConsoleColor BackgroundColor { get; set; }",
            "insertion_text": "BackgroundColor"
        },
        {
            "detailed_info": "void Beep(int frequency, int duration);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Beep(int frequency, int duration)",
            "insertion_text": "Beep("
        },
        {
            "detailed_info": "void Beep();\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Beep()",
            "insertion_text": "Beep()"
        },
        {
            "detailed_info": "int BufferHeight { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int BufferHeight { get; set; }",
            "insertion_text": "BufferHeight"
        },
        {
            "detailed_info": "int BufferWidth { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int BufferWidth { get; set; }",
            "insertion_text": "BufferWidth"
        },
        {
            "detailed_info": "ConsoleCancelEventHandler CancelKeyPress;\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "CancelKeyPress",
            "insertion_text": "CancelKeyPress"
        },
        {
            "detailed_info": "bool CapsLock { get; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "bool CapsLock { get; }",
            "insertion_text": "CapsLock"
        },
        {
            "detailed_info": "void Clear();\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Clear()",
            "insertion_text": "Clear()"
        },
        {
            "detailed_info": "int CursorLeft { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int CursorLeft { get; set; }",
            "insertion_text": "CursorLeft"
        },
        {
            "detailed_info": "int CursorSize { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int CursorSize { get; set; }",
            "insertion_text": "CursorSize"
        },
        {
            "detailed_info": "int CursorTop { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int CursorTop { get; set; }",
            "insertion_text": "CursorTop"
        },
        {
            "detailed_info": "bool CursorVisible { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "bool CursorVisible { get; set; }",
            "insertion_text": "CursorVisible"
        },
        {
            "detailed_info": "bool Equals(object objA, object objB);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "bool Equals(object objA, object objB)",
            "insertion_text": "Equals("
        },
        {
            "detailed_info": "TextWriter Error { get; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "TextWriter Error { get; }",
            "insertion_text": "Error"
        },
        {
            "detailed_info": "ConsoleColor ForegroundColor { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "ConsoleColor ForegroundColor { get; set; }",
            "insertion_text": "ForegroundColor"
        },
        {
            "detailed_info": "TextReader In { get; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "TextReader In { get; }",
            "insertion_text": "In"
        },
        {
            "detailed_info": "Encoding InputEncoding { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "Encoding InputEncoding { get; set; }",
            "insertion_text": "InputEncoding"
        },
        {
            "detailed_info": "bool IsErrorRedirected { get; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "bool IsErrorRedirected { get; }",
            "insertion_text": "IsErrorRedirected"
        },
        {
            "detailed_info": "bool IsInputRedirected { get; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "bool IsInputRedirected { get; }",
            "insertion_text": "IsInputRedirected"
        },
        {
            "detailed_info": "bool IsOutputRedirected { get; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "bool IsOutputRedirected { get; }",
            "insertion_text": "IsOutputRedirected"
        },
        {
            "detailed_info": "bool KeyAvailable { get; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "bool KeyAvailable { get; }",
            "insertion_text": "KeyAvailable"
        },
        {
            "detailed_info": "int LargestWindowHeight { get; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int LargestWindowHeight { get; }",
            "insertion_text": "LargestWindowHeight"
        },
        {
            "detailed_info": "int LargestWindowWidth { get; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int LargestWindowWidth { get; }",
            "insertion_text": "LargestWindowWidth"
        },
        {
            "detailed_info": "void MoveBufferArea(int sourceLeft, int sourceTop, int sourceWidth, int sourceHeight, int targetLeft, int targetTop);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void MoveBufferArea(int sourceLeft, int sourceTop, int sourceWidth, int sourceHeight, int targetLeft, int targetTop)",
            "insertion_text": "MoveBufferArea("
        },
        {
            "detailed_info": "void MoveBufferArea(int sourceLeft, int sourceTop, int sourceWidth, int sourceHeight, int targetLeft, int targetTop, char sourceChar, ConsoleColor sourceForeColor, ConsoleColor sourceBackColor);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void MoveBufferArea(int sourceLeft, int sourceTop, int sourceWidth, int sourceHeight, int targetLeft, int targetTop, char sourceChar, ConsoleColor sourceForeColor, ConsoleColor sourceBackColor)",
            "insertion_text": "MoveBufferArea("
        },
        {
            "detailed_info": "bool NumberLock { get; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "bool NumberLock { get; }",
            "insertion_text": "NumberLock"
        },
        {
            "detailed_info": "Stream OpenStandardError(int bufferSize);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "Stream OpenStandardError(int bufferSize)",
            "insertion_text": "OpenStandardError("
        },
        {
            "detailed_info": "Stream OpenStandardError();\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "Stream OpenStandardError()",
            "insertion_text": "OpenStandardError()"
        },
        {
            "detailed_info": "Stream OpenStandardInput(int bufferSize);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "Stream OpenStandardInput(int bufferSize)",
            "insertion_text": "OpenStandardInput("
        },
        {
            "detailed_info": "Stream OpenStandardInput();\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "Stream OpenStandardInput()",
            "insertion_text": "OpenStandardInput()"
        },
        {
            "detailed_info": "Stream OpenStandardOutput(int bufferSize);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "Stream OpenStandardOutput(int bufferSize)",
            "insertion_text": "OpenStandardOutput("
        },
        {
            "detailed_info": "Stream OpenStandardOutput();\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "Stream OpenStandardOutput()",
            "insertion_text": "OpenStandardOutput()"
        },
        {
            "detailed_info": "TextWriter Out { get; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "TextWriter Out { get; }",
            "insertion_text": "Out"
        },
        {
            "detailed_info": "Encoding OutputEncoding { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "Encoding OutputEncoding { get; set; }",
            "insertion_text": "OutputEncoding"
        },
        {
            "detailed_info": "int Read();\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int Read()",
            "insertion_text": "Read()"
        },
        {
            "detailed_info": "ConsoleKeyInfo ReadKey(bool intercept);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "ConsoleKeyInfo ReadKey(bool intercept)",
            "insertion_text": "ReadKey("
        },
        {
            "detailed_info": "ConsoleKeyInfo ReadKey();\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "ConsoleKeyInfo ReadKey()",
            "insertion_text": "ReadKey()"
        },
        {
            "detailed_info": "string ReadLine();\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "string ReadLine()",
            "insertion_text": "ReadLine()"
        },
        {
            "detailed_info": "bool ReferenceEquals(object objA, object objB);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "bool ReferenceEquals(object objA, object objB)",
            "insertion_text": "ReferenceEquals("
        },
        {
            "detailed_info": "void ResetColor();\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void ResetColor()",
            "insertion_text": "ResetColor()"
        },
        {
            "detailed_info": "void SetBufferSize(int width, int height);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void SetBufferSize(int width, int height)",
            "insertion_text": "SetBufferSize("
        },
        {
            "detailed_info": "void SetCursorPosition(int left, int top);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void SetCursorPosition(int left, int top)",
            "insertion_text": "SetCursorPosition("
        },
        {
            "detailed_info": "void SetError(TextWriter newError);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void SetError(TextWriter newError)",
            "insertion_text": "SetError("
        },
        {
            "detailed_info": "void SetIn(TextReader newIn);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void SetIn(TextReader newIn)",
            "insertion_text": "SetIn("
        },
        {
            "detailed_info": "void SetOut(TextWriter newOut);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void SetOut(TextWriter newOut)",
            "insertion_text": "SetOut("
        },
        {
            "detailed_info": "void SetWindowPosition(int left, int top);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void SetWindowPosition(int left, int top)",
            "insertion_text": "SetWindowPosition("
        },
        {
            "detailed_info": "void SetWindowSize(int width, int height);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void SetWindowSize(int width, int height)",
            "insertion_text": "SetWindowSize("
        },
        {
            "detailed_info": "string Title { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "string Title { get; set; }",
            "insertion_text": "Title"
        },
        {
            "detailed_info": "bool TreatControlCAsInput { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "bool TreatControlCAsInput { get; set; }",
            "insertion_text": "TreatControlCAsInput"
        },
        {
            "detailed_info": "int WindowHeight { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int WindowHeight { get; set; }",
            "insertion_text": "WindowHeight"
        },
        {
            "detailed_info": "int WindowLeft { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int WindowLeft { get; set; }",
            "insertion_text": "WindowLeft"
        },
        {
            "detailed_info": "int WindowTop { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int WindowTop { get; set; }",
            "insertion_text": "WindowTop"
        },
        {
            "detailed_info": "int WindowWidth { get; set; }\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "int WindowWidth { get; set; }",
            "insertion_text": "WindowWidth"
        },
        {
            "detailed_info": "void Write(bool value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(bool value)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(char value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(char value)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(char[] buffer);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(char[] buffer)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(decimal value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(decimal value)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(double value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(double value)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(int value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(int value)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(long value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(long value)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(object value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(object value)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(float value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(float value)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(string value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(string value)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(uint value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(uint value)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(ulong value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(ulong value)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(string format, object arg0);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(string format, object arg0)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(string format, params object[] arg);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(string format, params object[] arg)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(char[] buffer, int index, int count);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(char[] buffer, int index, int count)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(string format, object arg0, object arg1);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(string format, object arg0, object arg1)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(string format, object arg0, object arg1, object arg2);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(string format, object arg0, object arg1, object arg2)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void Write(string format, object arg0, object arg1, object arg2, object arg3);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void Write(string format, object arg0, object arg1, object arg2, object arg3)",
            "insertion_text": "Write("
        },
        {
            "detailed_info": "void WriteLine(bool value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(bool value)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(char value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(char value)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(char[] buffer);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(char[] buffer)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(decimal value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(decimal value)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(double value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(double value)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(int value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(int value)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(long value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(long value)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(object value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(object value)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(float value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(float value)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(string value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(string value)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(uint value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(uint value)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(ulong value);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(ulong value)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(string format, object arg0);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(string format, object arg0)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(string format, params object[] arg);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(string format, params object[] arg)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(char[] buffer, int index, int count);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(char[] buffer, int index, int count)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(string format, object arg0, object arg1);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(string format, object arg0, object arg1)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(string format, object arg0, object arg1, object arg2);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(string format, object arg0, object arg1, object arg2)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine(string format, object arg0, object arg1, object arg2, object arg3);\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine(string format, object arg0, object arg1, object arg2, object arg3)",
            "insertion_text": "WriteLine("
        },
        {
            "detailed_info": "void WriteLine();\n",
            "extra_data": {
                "required_namespace_import": null
            },
            "extra_menu_info": "void WriteLine()",
            "insertion_text": "WriteLine()"
        }
    ],
    "errors": []
}
```

==== Sending defined subcommands request ====
```http
POST /defined_subcommands HTTP/1.1
Accept: application/json
Accept-Encoding: gzip, deflate
Connection: keep-alive
Content-Length: 142
Host: 127.0.0.1:53650
User-Agent: HTTPie/0.9.2
X-Ycm-Hmac: aO/vKlvOeIM/J5FXsEU5gLIYwQzCA5in1HcwUmUM+Q0=
content-type: application/json

{
    "column_num": null,
    "completer_target": "python",
    "file_data": {
        "": {
            "contents": "",
            "filetypes": [
                null
            ]
        }
    },
    "filepath": "",
    "line_num": null
}

HTTP/1.1 200 OK
Content-Length: 55
Content-Type: application/json
Date: Wed, 04 Nov 2015 18:09:43 GMT
Server: waitress
X-Ycm-Hmac: GtL3ufW2dS2anNKuTT7fAsMcrsvrR5GZIqL1dpgf7CU=

[
    "GoToDefinition",
    "GoToDeclaration",
    "GoTo",
    "GetDoc"
]
```

==== Sending GoTo request ====
```http
POST /run_completer_command HTTP/1.1
Accept: application/json
Accept-Encoding: gzip, deflate
Connection: keep-alive
Content-Length: 1031
Host: 127.0.0.1:53650
User-Agent: HTTPie/0.9.2
X-Ycm-Hmac: ju2Wx3Y5hYWPKDnVoX+bPyuVxHHemVqZ7G2VXPcexv8=
content-type: application/json

{
    "column_num": 4,
    "command_arguments": [
        "GoTo"
    ],
    "file_data": {
        "/Users/jwilm/code/ycmd/examples/samples/some_cpp.cpp": {
            "contents": "// Copyright (C) 2014  Google Inc.\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//     http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nstruct Foo {\n  int x;\n  int y  // There's a missing semicolon here\n  char c;\n};\n\nint main()\n{\n  Foo foo;\n  // The location after the dot is line 25, col 7\n  foo.\n}\n",
            "filetypes": [
                "cpp"
            ]
        }
    },
    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_cpp.cpp",
    "line_num": 23
}

HTTP/1.1 200 OK
Content-Length: 101
Content-Type: application/json
Date: Wed, 04 Nov 2015 18:09:44 GMT
Server: waitress
X-Ycm-Hmac: I43VGpuqvUZWBG58CV8a22Vb4fhYK681FT6voxKHXuE=

{
    "column_num": 8,
    "filepath": "/Users/jwilm/code/ycmd/examples/samples/some_cpp.cpp",
    "line_num": 15
}
```

Shutting down server...
