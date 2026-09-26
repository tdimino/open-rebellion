
undefined4 __thiscall FUN_004faf50(void *this,void *param_1,int *param_2,void *param_3)

{
  void *pvVar1;
  int iVar2;
  uint uVar3;
  void *pvVar4;
  uint *puVar5;
  uint *puVar6;
  undefined4 uVar7;
  bool bVar8;
  void *pvVar9;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  pvVar1 = param_3;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00640048;
  local_c = ExceptionList;
  if ((param_1 == (void *)0x0) || (param_2 == (int *)0x0)) {
    uVar7 = 0;
  }
  else {
    ExceptionList = &local_c;
    iVar2 = FUN_004f78e0(this,0,param_3);
    bVar8 = iVar2 != 0;
    if (((*(byte *)((int)this + 0x50) & 0x20) != 0) &&
       (uVar3 = *(uint *)((int)this + 0x40) >> 0x10 & 0xff, (int)uVar3 < DAT_0065d254)) {
      iVar2 = FUN_004f72f0(this,uVar3 + 1,pvVar1);
      if ((iVar2 == 0) || (!bVar8)) {
        bVar8 = false;
      }
      else {
        bVar8 = true;
      }
    }
    iVar2 = (**(code **)(*(int *)this + 0x8c))(pvVar1);
    if ((iVar2 == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    iVar2 = (**(code **)(*(int *)this + 0x90))(pvVar1);
    if ((iVar2 == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    iVar2 = FUN_004f8240(this,pvVar1);
    if ((iVar2 == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    iVar2 = FUN_004f81c0(this,pvVar1);
    if ((iVar2 == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    iVar2 = FUN_004f80e0(this,pvVar1);
    if ((iVar2 == 0) || (!bVar8)) {
      iVar2 = 0;
    }
    else {
      iVar2 = 1;
    }
    if ((*(byte *)((int)this + 0x50) & 8) != 0) {
      bVar8 = false;
      uStack_4 = 0;
      uVar3 = (**(code **)(*param_2 + 4))();
      if ((0xf1 < uVar3) && (uVar3 < 0xf3)) {
        bVar8 = true;
      }
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (bVar8) {
        pvVar9 = pvVar1;
        pvVar4 = (void *)FUN_00539fd0(this,1);
        FUN_004fc710(pvVar4,pvVar9);
        pvVar9 = pvVar1;
        pvVar4 = (void *)FUN_00539fd0(this,2);
        iVar2 = FUN_004fc710(pvVar4,pvVar9);
      }
    }
    puVar5 = FUN_004025b0(param_2,(uint *)&param_3);
    uStack_4 = 1;
    puVar6 = FUN_004025b0(param_1,(uint *)&param_2);
    uStack_4._0_1_ = 2;
    uVar3 = FUN_0053f9f0(800,this,puVar6,puVar5,pvVar1);
    if ((uVar3 == 0) || (iVar2 == 0)) {
      uVar7 = 0;
    }
    else {
      uVar7 = 1;
    }
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    FUN_00619730();
    uStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return uVar7;
}

