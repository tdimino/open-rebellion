
uint FUN_0052eb60(int *param_1,int *param_2)

{
  bool bVar1;
  int *piVar2;
  int *this;
  int iVar3;
  uint uVar4;
  uint uVar5;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar2 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006452c0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  iVar3 = thunk_FUN_00506e60();
  uVar5 = (uint)(iVar3 != 0);
  if (uVar5 != 0) {
    uVar5 = 0;
    local_4 = 0;
    uVar4 = (**(code **)(*param_1 + 4))();
    if ((0x27 < uVar4) && (uVar4 < 0x2c)) {
      uVar5 = 1;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if (uVar5 != 0) {
    param_2 = (int *)0x0;
    uVar5 = FUN_00555540(param_1,(int *)&param_2);
    if ((uVar5 != 0) && (param_2 != (int *)0x0)) {
      bVar1 = true;
      local_4 = 1;
      uVar4 = (**(code **)(*param_2 + 4))();
      if ((uVar4 < 0x90) || (0x97 < uVar4)) {
        bVar1 = false;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      this = param_2;
      if (bVar1) {
        iVar3 = (**(code **)(*param_1 + 0x204))();
        iVar3 = FUN_00509670(this,iVar3);
        *piVar2 = iVar3;
        uVar5 = (uint)(iVar3 != 0);
      }
    }
  }
  ExceptionList = pvStack_c;
  return uVar5;
}

