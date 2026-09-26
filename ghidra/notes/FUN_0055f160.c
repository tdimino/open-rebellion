
int FUN_0055f160(uint *param_1,int param_2,void *param_3)

{
  void *this;
  int *piVar1;
  uint uVar2;
  int iVar3;
  int iVar4;
  bool bVar5;
  int iStack_20;
  uint local_1c;
  uint local_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_3;
  puStack_8 = &LAB_00649fa0;
  local_c = ExceptionList;
  iVar4 = 1;
  local_1c = 1;
  local_18 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar1 = (int *)FUN_00504dc0(param_1);
  if (piVar1 != (int *)0x0) {
    local_1c = 0x30;
    local_18 = 0x3c;
    local_4 = 1;
    uVar2 = (**(code **)(*piVar1 + 4))();
    if ((uVar2 < local_1c) || (local_18 <= uVar2)) {
      iVar4 = 0;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    bVar5 = iVar4 != 0;
    iVar4 = 0;
    if (bVar5) {
      local_1c = 0;
      param_3 = (void *)0x0;
      param_1 = (uint *)0x0;
      iStack_20 = 0;
      iVar3 = FUN_0053ebb0(*(undefined4 *)(param_2 + 0xc),&iStack_20);
      iVar4 = 0;
      if (((iVar3 != 0) &&
          (iVar4 = FUN_0055ed30(piVar1,(int *)&local_1c,(int *)&param_1,&param_3), iVar4 != 0)) &&
         (param_3 == (void *)0x0)) {
        uStack_14 = 0x30;
        uStack_10 = 4;
        local_4 = 2;
        FUN_00520580(this,&uStack_14);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
      if (iVar4 != 0) {
        if (((*(int *)((int)this + 4) == -1) && (param_1 == (uint *)0x0)) && (iStack_20 == 2)) {
          uStack_14 = 0x30;
          uStack_10 = 5;
          local_4 = 3;
          FUN_00520580(this,&uStack_14);
          local_4 = 0xffffffff;
          FUN_00619730();
        }
        if ((iVar4 != 0) && (*(int *)((int)this + 4) == -1)) {
          iVar4 = (**(code **)(*piVar1 + 0x2d0))(param_2,this);
        }
      }
    }
  }
  ExceptionList = local_c;
  return iVar4;
}

