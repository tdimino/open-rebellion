
int FUN_00561990(int *param_1,int param_2,void *param_3)

{
  uint *puVar1;
  uint uVar2;
  int *this;
  bool bVar3;
  undefined3 extraout_var;
  int *piVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 auStack_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  this = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a2d0;
  local_c = ExceptionList;
  iVar6 = 0;
  iVar8 = 1;
  if (param_2 != 0) {
    uVar2 = param_1[0x2b];
    local_50 = 0;
    param_2 = 0;
    ExceptionList = &local_c;
    bVar3 = FUN_004f6b50(param_1,&param_2);
    if ((CONCAT31(extraout_var,bVar3) == 0) || (bVar3 = true, (uVar2 >> 8 & 1) == 0)) {
      bVar3 = false;
    }
    puVar1 = (uint *)(param_1 + 0x2b);
    iVar7 = 0;
    iVar8 = 0;
    param_1 = (int *)0x0;
    if (((*puVar1 & 0x100) != 0) && (param_2 != 0)) {
      FUN_004f2640(local_4c,param_2,1,(uint)this[9] >> 6 & 3);
      local_4 = 0;
      FUN_00513120((int)local_4c);
      while (local_30 != 0) {
        piVar4 = (int *)FUN_0052bed0((int)local_4c);
        iVar7 = (**(code **)(*piVar4 + 0x1d4))();
        if (iVar7 == 0) {
          iVar8 = iVar8 + 1;
          piVar4 = (int *)FUN_0052bed0((int)local_4c);
          iVar7 = (**(code **)(*piVar4 + 0x1f0))();
          iVar6 = iVar6 + iVar7;
        }
        FUN_005130d0((int)local_4c);
      }
      if (iVar8 != 0) {
        param_1 = (int *)(iVar6 / iVar8);
      }
      FUN_00504cc0(auStack_2c,param_2,1,(uint)this[9] >> 6 & 3);
      local_4._0_1_ = 1;
      iVar7 = FUN_00513180((int)auStack_2c);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00504d40(auStack_2c);
      local_4 = 0xffffffff;
      FUN_004f26c0(local_4c);
    }
    piVar4 = &local_50;
    iVar6 = (**(code **)(*this + 0x1e0))();
    iVar8 = (**(code **)(*this + 0x1f0))();
    uVar5 = FUN_004edc80((int)this);
    iVar6 = FUN_0055e7e0(uVar5,iVar8,iVar6,(int)param_1,iVar7,piVar4);
    if ((iVar6 == 0) || (!bVar3)) {
      iVar8 = 0;
    }
    else {
      iVar8 = 1;
    }
    if (local_50 != 0) {
      iVar6 = FUN_004eeb80(this,1,param_3);
      if ((iVar6 == 0) || (iVar8 == 0)) {
        iVar8 = 0;
      }
      else {
        iVar8 = 1;
      }
    }
  }
  ExceptionList = local_c;
  return iVar8;
}

