
undefined4 __fastcall FUN_0057c520(void *param_1)

{
  int *piVar1;
  void *pvVar2;
  undefined4 local_14;
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064dd28;
  local_c = ExceptionList;
  local_14 = 0;
  ExceptionList = &local_c;
  piVar1 = (int *)thunk_FUN_00506e80();
  if (piVar1 != (int *)0x0) {
    pvVar2 = FUN_004fa150(param_1,local_10);
    local_4 = 0;
    (**(code **)(*piVar1 + 0x1bc))(pvVar2,(int)param_1 + 0x40,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return local_14;
}

