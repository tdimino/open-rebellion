
uint __thiscall FUN_0052a8f0(void *this,void *param_1)

{
  int *piVar1;
  uint *puVar2;
  int iVar3;
  uint uVar4;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006449a0;
  local_c = ExceptionList;
  uVar4 = 1;
  iVar3 = 0;
  ExceptionList = &local_c;
  local_10 = this;
  piVar1 = (int *)FUN_00527c00(this,&local_10);
  local_4 = 0;
  piVar1 = FUN_004f26e0(piVar1);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (piVar1 != (int *)0x0) {
    puVar2 = (uint *)FUN_00527c00(this,&local_10);
    local_4 = 1;
    iVar3 = FUN_0051cab0(puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar4 = (uint)(iVar3 != 0);
    if (uVar4 == 0) {
      ExceptionList = local_c;
      return 0;
    }
    iVar3 = FUN_0053b860(iVar3);
  }
  if (uVar4 != 0) {
    uVar4 = FUN_00529650(this,iVar3,param_1);
  }
  ExceptionList = local_c;
  return uVar4;
}

