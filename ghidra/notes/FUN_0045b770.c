
void __thiscall FUN_0045b770(void *this,uint *param_1,int *param_2)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  uint uVar4;
  int *this_00;
  uint local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006309e0;
  local_c = ExceptionList;
  this_00 = (int *)param_2[7];
  uVar4 = *param_1 >> 0x18;
  ExceptionList = &local_c;
  FUN_004025b0(this_00,local_28);
  bVar1 = false;
  local_4 = 0;
  local_28[1] = 0x90;
  local_28[2] = 0x98;
  if ((0x8f < uVar4) && (uVar4 < 0x98)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    iVar3 = *(int *)(*(int *)((int)this + 0x194) + 0x9c);
    if (param_2 == (int *)0x0) {
      param_2 = FUN_004f3220(iVar3,param_1);
    }
    iVar3 = FUN_00507270(param_2,iVar3);
    FUN_0045baf0(this,param_1,100,iVar3);
    FUN_0045c240(this,param_1,param_2[0x17],param_2[0x18]);
    FUN_0045c450(this,param_1,param_2[0x19],param_2[0x1a]);
    FUN_0045bbb0(this,param_1,(uint)param_2[9] >> 6 & 3);
    FUN_0045ccc0(this,param_2);
    FUN_0045cdc0(this,param_2);
    FUN_0045ce80(this,param_2);
    this_00 = param_2;
  }
  else {
    local_28[4] = 0x30;
    local_28[3] = 0x28;
    local_4 = CONCAT31(local_4._1_3_,1);
    if ((uVar4 < 0x28) || (0x2f < uVar4)) {
LAB_0045b8d3:
      bVar2 = false;
    }
    else {
      local_28[1] = 0x90;
      local_28[2] = 0x98;
      if ((local_28[0] >> 0x18 < 0x90) || (0x97 < local_28[0] >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      bVar2 = true;
      if (!bVar1) goto LAB_0045b8d3;
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00619730();
    if (bVar2) {
      FUN_0045cdc0(this,this_00);
      goto LAB_0045bac8;
    }
    local_28[5] = 0x1c;
    local_28[6] = 0x20;
    local_4 = CONCAT31(local_4._1_3_,2);
    if ((uVar4 < 0x1c) || (0x1f < uVar4)) {
      local_28[3] = 0x22;
      local_28[4] = 0x28;
      if ((uVar4 < 0x22) || (0x27 < uVar4)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0045b983;
      local_28[1] = 0x10;
      local_28[2] = 0x14;
      if ((uVar4 < 0x10) || (0x13 < uVar4)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      bVar2 = false;
      if (bVar1) goto LAB_0045b983;
    }
    else {
LAB_0045b983:
      bVar2 = true;
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00619730();
    if (bVar2) {
      FUN_0045ce80(this,this_00);
      goto LAB_0045bac8;
    }
    local_28[5] = 0x30;
    local_28[6] = 0x40;
    if ((uVar4 < 0x30) || (0x3f < uVar4)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_0045ce80(this,this_00);
      goto LAB_0045bac1;
    }
    local_28[3] = 8;
    local_28[4] = 0x10;
    local_4 = CONCAT31(local_4._1_3_,3);
    if ((uVar4 < 8) || (0xf < uVar4)) {
LAB_0045ba5b:
      bVar2 = false;
    }
    else {
      local_28[5] = 0x90;
      local_28[6] = 0x98;
      if ((local_28[0] >> 0x18 < 0x90) || (0x97 < local_28[0] >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      bVar2 = true;
      if (!bVar1) goto LAB_0045ba5b;
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00619730();
    if (bVar2) {
      FUN_0045ccc0(this,this_00);
      goto LAB_0045bac8;
    }
    local_28[5] = 0x40;
    local_28[6] = 0x80;
    if ((uVar4 < 0x40) || (0x7f < uVar4)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) goto LAB_0045bac8;
  }
LAB_0045bac1:
  FUN_0045d090(this,this_00);
LAB_0045bac8:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

