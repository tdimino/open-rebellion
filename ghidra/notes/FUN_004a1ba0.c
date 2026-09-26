
void __thiscall FUN_004a1ba0(void *this,uint param_1,int param_2)

{
  int *piVar1;
  int *this_00;
  undefined4 *puVar2;
  void *pvVar3;
  uint *puVar4;
  bool bVar5;
  uint uVar6;
  uint uStack_3c;
  int iStack_38;
  int *local_34;
  uint local_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *local_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_006377d3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar1 = FUN_004a25c0(this);
  if ((piVar1 != (int *)0x0) &&
     (((*(uint *)((int)this + 0x160) != param_1 || (param_2 != 0)) &&
      (*(int *)((int)this + 0x1c4) != 0)))) {
    *(uint *)((int)this + 0x160) = param_1;
    local_34 = piVar1;
    FUN_0060d7e0(*(void **)((int)this + 0x15c),param_1,1);
    local_30 = (uint)(*(int *)((int)this + 0x160) == 0x18);
    this_00 = (int *)(**(code **)(**(int **)((int)this + 0x1bc) + 8))();
    FUN_004ece30(&uStack_3c);
    iStack_4 = 0;
    if (this_00 != (int *)0x0) {
      do {
        puVar2 = FUN_0042d170(this_00,&param_1);
        iStack_4._0_1_ = 1;
        FUN_004f26d0(&uStack_3c,puVar2);
        iStack_4 = (uint)iStack_4._1_3_ << 8;
        FUN_00619730();
        puVar2 = FUN_004f5940((void *)((int)this + 0x114),&uStack_3c);
        if (puVar2 != (undefined4 *)0x0) {
          FUN_004acca0((void *)((int)this + 0x114),puVar2);
        }
        this_00 = (int *)(**(code **)(*this_00 + 0xc))();
        piVar1 = local_34;
      } while (this_00 != (int *)0x0);
    }
    FUN_005f5b20(*(void **)((int)this + 0x1bc));
    FUN_00536da0(auStack_2c,piVar1,3);
    iStack_4._0_1_ = 2;
    FUN_0042d170(*(void **)((int)this + 0x1c4),&iStack_38);
    iStack_4._0_1_ = 3;
    FUN_00513120((int)auStack_2c);
    while (iStack_10 != 0) {
      pvVar3 = (void *)FUN_0052bed0((int)auStack_2c);
      puVar4 = FUN_004025b0(pvVar3,&param_1);
      iStack_4._0_1_ = 4;
      FUN_004f26d0(&uStack_3c,puVar4);
      iStack_4._0_1_ = 3;
      FUN_00619730();
      piVar1 = FUN_0042d170(pvVar3,&param_2);
      bVar5 = *piVar1 == iStack_38;
      FUN_00619730();
      if (bVar5) {
        local_34 = (int *)FUN_00618b70(0x28);
        iStack_4._0_1_ = 5;
        if (local_34 == (int *)0x0) {
          puVar2 = (undefined4 *)0x0;
        }
        else {
          puVar2 = FUN_004acd80(local_34,&uStack_3c,0);
        }
        iStack_4._0_1_ = 3;
        FUN_004acba0((void *)((int)this + 0x114),puVar2,0);
        if (local_30 == 0) {
LAB_004a1d7e:
          if ((*(byte *)((int)pvVar3 + 0x78) & 1) != 0) goto LAB_004a1da9;
        }
        else if ((*(byte *)((int)pvVar3 + 0x78) & 1) == 0) {
          if (local_30 != 0) goto LAB_004a1da9;
          goto LAB_004a1d7e;
        }
        uVar6 = 0;
        pvVar3 = (void *)0x1;
        piVar1 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x14c) + 0x9c),&uStack_3c);
        FUN_004a0e10(this,piVar1,pvVar3,uVar6);
      }
LAB_004a1da9:
      FUN_005130d0((int)auStack_2c);
    }
    FUN_0060a280(*(void **)((int)this + 0x1c0));
    iStack_4._0_1_ = 2;
    FUN_00619730();
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_00536ea0(auStack_2c);
    iStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return;
}

