
undefined4 * __thiscall FUN_0060ed60(undefined4 *param_1,LPCSTR param_2)

{
  HMODULE pHVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656ce8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5b80();
  local_4 = 0;
  *param_1 = &PTR_FUN_0066e280;
  pHVar1 = LoadLibraryA(param_2);
  param_1[4] = pHVar1;
  if (pHVar1 == (HMODULE)0x0) {
    GetLastError();
  }
  ExceptionList = local_c;
  return param_1;
}

