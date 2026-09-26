
undefined4 __thiscall FUN_004fb310(void *this,undefined4 param_1,void *param_2)

{
  void *pvVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  uint uVar4;
  bool bVar5;
  int unaff_retaddr;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  pvVar1 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006400b0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = this;
  iVar3 = FUN_004f9a60(this,(int)param_2);
  bVar5 = iVar3 != 0;
  bVar2 = FUN_005406d0(this);
  if (CONCAT31(extraout_var,bVar2) != 0) {
    FUN_004ece30(&param_2);
    local_4 = 0;
    FUN_004ece30(&local_10);
    local_4 = CONCAT31(local_4._1_3_,1);
    iVar3 = FUN_004f6b70(this,(uint *)&param_2);
    if ((iVar3 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    iVar3 = FUN_004f6e30(this,&local_10);
    if ((iVar3 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    if (bVar5) {
      iVar3 = FUN_004ece60((uint *)&param_2);
      if ((iVar3 != 0) && (param_2 != local_10)) {
        uVar4 = *(uint *)((int)this + 0x40) >> 8 & 0xff;
        bVar5 = true;
        if ((int)uVar4 < DAT_0065d250) {
          iVar3 = FUN_004f7260(this,uVar4 + 1,pvVar1);
          bVar5 = iVar3 != 0;
        }
        iVar3 = FUN_004f7fa0(this,&param_2);
        if ((iVar3 == 0) || (!bVar5)) {
          bVar5 = false;
        }
        else {
          bVar5 = true;
        }
      }
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  iVar3 = (**(code **)(*(int *)this + 0xe0))(pvVar1);
  if ((iVar3 == 0) || (!bVar5)) {
    bVar5 = false;
  }
  else {
    bVar5 = true;
  }
  iVar3 = FUN_0053f9c0(0x301,this,unaff_retaddr,pvVar1);
  if ((iVar3 != 0) && (bVar5)) {
    ExceptionList = local_10;
    return 1;
  }
  ExceptionList = local_10;
  return 0;
}

