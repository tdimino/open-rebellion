
uint FUN_00556c70(uint *param_1,int param_2,int param_3)

{
  int *this;
  int iVar1;
  undefined3 extraout_var;
  uint uVar2;
  void *this_00;
  undefined3 extraout_var_00;
  uint uVar3;
  bool bVar4;
  int *local_40;
  int *local_3c;
  uint uStack_38;
  uint uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649720;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  this = (int *)FUN_00504dc0(param_1);
  local_3c = this;
  FUN_004ece30(&param_1);
  local_4 = 0;
  iVar1 = 0;
  if (this != (int *)0x0) {
    iVar1 = (**(code **)(*this + 0xc))(&param_1);
  }
  bVar4 = iVar1 == 0;
  local_40 = (int *)0x0;
  uVar3 = 0;
  if (!bVar4) {
    bVar4 = FUN_004f6b50(this,(int *)&local_40);
    uVar3 = CONCAT31(extraout_var,bVar4);
    bVar4 = uVar3 == 0;
  }
  iVar1 = param_2;
  if (bVar4) goto LAB_00556d9b;
  uStack_30 = 0x90;
  uStack_2c = 0x98;
  if (((uint)param_1 >> 0x18 < 0x90) || (0x97 < (uint)param_1 >> 0x18)) {
LAB_00556d58:
    bVar4 = false;
  }
  else {
    uStack_38 = 0x90;
    uStack_34 = 0x98;
    local_4._0_1_ = 2;
    uVar2 = (**(code **)(*local_40 + 4))();
    if ((uVar2 < uStack_38) || (uStack_34 <= uVar2)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    if (bVar4) goto LAB_00556d58;
    bVar4 = true;
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  this = local_3c;
  if (bVar4) {
    this_00 = (void *)FUN_00505f60((uint *)&param_1);
    uVar3 = 0;
    this = local_3c;
    if (this_00 != (void *)0x0) {
      bVar4 = FUN_0050d5a0(this_00,5,iVar1,param_3);
      uVar3 = CONCAT31(extraout_var_00,bVar4);
      this = local_3c;
    }
  }
LAB_00556d9b:
  if (uVar3 != 0) {
    FUN_004fd450(&uStack_28,param_3);
    local_4._0_1_ = 3;
    uStack_24 = 1;
    uStack_20 = 3;
    if ((iVar1 == 1) || (iVar1 == 2)) {
      uVar3 = FUN_004f9510(this,iVar1,&uStack_28);
    }
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_004fd4d0(&uStack_28);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return uVar3;
}

