
void __thiscall FUN_0048aa00(void *this,undefined4 *param_1)

{
  int *piVar1;
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634f78;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar1 = (int *)FUN_004f5cd0(0x205);
  if (piVar1 != (int *)0x0) {
    FUN_004f4340(local_14);
    local_4 = 0;
    FUN_004f44b0(local_14,param_1,0);
    piVar1[8] = *(int *)((int)this + 0x24);
    (**(code **)(*piVar1 + 0x24))(local_14);
    FUN_0048aa90(this,piVar1);
    local_4 = 0xffffffff;
    FUN_004f4380(local_14);
  }
  ExceptionList = local_c;
  return;
}

