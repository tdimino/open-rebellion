
uint __thiscall FUN_0057dba0(void *this,void *param_1)

{
  void *pvVar1;
  int *piVar2;
  void *pvVar3;
  uint uVar4;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  puStack_8 = &LAB_0064e190;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 1;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar2 = (int *)thunk_FUN_00506e80();
  uVar4 = 0;
  if (piVar2 != (int *)0x0) {
    pvVar3 = FUN_004fa150(this,&param_1);
    local_4 = 1;
    uVar4 = (**(code **)(*piVar2 + 0x1c8))(pvVar3,(int)this + 0x40,pvVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return uVar4;
}

