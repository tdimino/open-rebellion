
void __thiscall FUN_004486f0(void *this,void *param_1,void *param_2)

{
  ushort uVar1;
  int *this_00;
  void *pvVar2;
  void *pvVar3;
  char *pcVar4;
  int *piVar5;
  undefined *puVar6;
  uint *puVar7;
  undefined4 uVar8;
  int iVar9;
  int iVar10;
  int local_5c;
  undefined4 local_58;
  undefined4 local_54;
  int local_50;
  int local_4c;
  undefined4 local_48 [3];
  undefined4 local_3c [3];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f170;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_48);
  local_4 = 0;
  local_58 = (int *)CONCAT22(DAT_0065d424,0x8723);
  FUN_005f3010(local_48,&local_58);
  pcVar4 = (char *)FUN_00583c40((int)local_48);
  FUN_00601aa0((void *)((int)this + 0x130),pcVar4);
  FUN_005f2f50(local_18);
  pvVar3 = param_2;
  pvVar2 = param_1;
  this_00 = *(int **)((int)this + 300);
  local_4._0_1_ = 1;
  FUN_0044a210((int)this);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8724);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(2,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  piVar5 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x114) + 0x9c),
                        (uint *)(*(int *)((int)this + 300) + 0xa8));
  if (piVar5 == (int *)0x0) {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8754);
    FUN_005f3010(local_48,&param_1);
  }
  else {
    puVar6 = FUN_004f62d0((int)piVar5);
    pcVar4 = (char *)FUN_00583c40((int)puVar6);
    FUN_005f35b0(local_3c,pcVar4);
    local_4._0_1_ = 2;
    FUN_005f30d0(local_48,(int)local_3c);
    local_4._0_1_ = 1;
    FUN_005f2ff0(local_3c);
  }
  FUN_0044b1d0(2,(int)local_48,pvVar3);
  FUN_0044a2e0(this,3,pvVar2,pvVar3);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8666);
  FUN_005f3010(local_48,&param_1);
  iVar10 = 4;
  FUN_0044b1d0(4,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  FUN_004ece30(&local_54);
  local_4._0_1_ = 3;
  puVar7 = (uint *)FUN_0042d170(this_00,&param_1);
  local_4._0_1_ = 4;
  local_58 = FUN_004f3000(*(int *)(*(int *)((int)this + 0x114) + 0x9c),puVar7);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_00619730();
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8748);
  FUN_005f3010(local_48,&param_1);
  if ((*(byte *)(this_00 + 0x14) & 0x10) == 0) {
    if ((*(byte *)(this_00 + 0x2b) & 1) != 0) {
      param_1 = (void *)CONCAT22(DAT_0065d424,0x8746);
      goto LAB_0044897b;
    }
    if ((short)this_00[0x25] != 0) {
      param_1 = (void *)CONCAT22(DAT_0065d424,0x8747);
      goto LAB_0044897b;
    }
    if ((local_58 != (int *)0x0) && (iVar9 = FUN_00520b70((int)local_58), iVar9 == 0)) {
      param_1 = (void *)CONCAT22(DAT_0065d424,0x8745);
      goto LAB_0044897b;
    }
  }
  else {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8744);
LAB_0044897b:
    FUN_005f3010(local_48,&param_1);
  }
  FUN_0044b1d0(4,(int)local_48,param_2);
  local_50 = 1;
  local_4c = -1;
  local_4._0_1_ = 5;
  local_5c = 0;
  FUN_004fd2b0(*(int **)((int)this + 300),&local_50,&local_5c);
  if ((local_4c == -1) && (local_5c != 0)) {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x3814);
    FUN_005f2fc0(local_24,&param_1);
    local_4._0_1_ = 6;
    FUN_005f3c20(local_48);
    param_1 = (void *)CONCAT22(param_1._2_2_,0x8766);
    FUN_005f2fc0(local_30,&param_1);
    local_4._0_1_ = 7;
    pcVar4 = (char *)FUN_00583c40((int)local_30);
    FUN_005f35b0(local_3c,pcVar4);
    local_4._0_1_ = 8;
    FUN_005f30d0(local_48,(int)local_3c);
    local_4._0_1_ = 7;
    FUN_005f2ff0(local_3c);
    iVar10 = 5;
    FUN_0044b1d0(5,(int)local_48,pvVar2);
    FUN_005f3c20(local_48);
    pcVar4 = (char *)FUN_00583c40((int)local_24);
    FUN_005f35b0(local_3c,pcVar4);
    local_4._0_1_ = 9;
    FUN_005f30d0(local_48,(int)local_3c);
    FUN_005f31a0(local_48,local_5c);
    local_4._0_1_ = 7;
    FUN_005f2ff0(local_3c);
    FUN_0044b1d0(5,(int)local_48,param_2);
    FUN_005f3c20(local_48);
    local_4._0_1_ = 6;
    FUN_005f2ff0(local_30);
    local_4._0_1_ = 5;
    FUN_005f2ff0(local_24);
  }
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8725);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 1,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  uVar1 = *(ushort *)(this_00 + 0x23);
  if (uVar1 < 10) {
    if (0x13 < uVar1) goto LAB_00448b61;
    if (0x4f < uVar1) goto LAB_00448b84;
    if (99 < uVar1) goto LAB_00448ba6;
    if (uVar1 < 0x78) {
      param_1._0_2_ = 0x8754;
      goto LAB_00448bf8;
    }
  }
  else {
    if (uVar1 < 0x14) {
      param_1 = (void *)CONCAT22(DAT_0065d424,0x8749);
      goto LAB_00448bf9;
    }
LAB_00448b61:
    if (uVar1 < 0x50) {
      param_1 = (void *)CONCAT22(DAT_0065d424,0x8750);
      goto LAB_00448bf9;
    }
LAB_00448b84:
    if (uVar1 < 100) {
      param_1._0_2_ = 0x8751;
LAB_00448bf8:
      param_1 = (void *)CONCAT22(DAT_0065d424,param_1._0_2_);
      goto LAB_00448bf9;
    }
LAB_00448ba6:
    if (uVar1 < 0x78) {
      param_1 = (void *)CONCAT22(DAT_0065d424,0x8752);
      goto LAB_00448bf9;
    }
  }
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8753);
LAB_00448bf9:
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 1,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8726);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 2,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  uVar8 = (**(code **)(*this_00 + 0x1dc))();
  FUN_005f31f0(local_48,uVar8);
  FUN_0044b1d0(iVar10 + 2,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8727);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 3,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  uVar8 = (**(code **)(*this_00 + 0x1e0))();
  FUN_005f31f0(local_48,uVar8);
  FUN_0044b1d0(iVar10 + 3,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8728);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 4,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  uVar8 = (**(code **)(*this_00 + 0x1f0))();
  FUN_005f31f0(local_48,uVar8);
  FUN_0044b1d0(iVar10 + 4,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8729);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 5,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  uVar8 = (**(code **)(*this_00 + 500))();
  FUN_005f31f0(local_48,uVar8);
  FUN_0044b1d0(iVar10 + 5,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8730);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 6,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  FUN_005f35b0(local_24,(char *)&lpString_006a7f40);
  local_4._0_1_ = 10;
  FUN_005f30d0(local_48,(int)local_24);
  local_4._0_1_ = 5;
  FUN_005f2ff0(local_24);
  FUN_0044b1d0(iVar10 + 6,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8731);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 7,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  iVar9 = (**(code **)(*this_00 + 0x1e4))();
  if (iVar9 == 0) {
    param_1._0_2_ = 0x8755;
  }
  else {
    param_1._0_2_ = 0x8756;
  }
  param_1 = (void *)CONCAT22(DAT_0065d424,param_1._0_2_);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 7,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8732);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 8,(int)local_48,pvVar2);
  iVar9 = (**(code **)(*this_00 + 0x1e8))();
  if (iVar9 == 0) {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8755);
  }
  else {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8756);
  }
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 8,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8733);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 9,(int)local_48,pvVar2);
  iVar9 = (**(code **)(*this_00 + 0x1ec))();
  if (iVar9 == 0) {
    param_1._0_2_ = 0x8755;
  }
  else {
    param_1._0_2_ = 0x8756;
  }
  param_1 = (void *)CONCAT22(DAT_0065d424,param_1._0_2_);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 9,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8737);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 10,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  FUN_005f35b0(local_24,(char *)&lpString_006a7f40);
  local_4._0_1_ = 0xb;
  FUN_005f30d0(local_48,(int)local_24);
  local_4._0_1_ = 5;
  FUN_005f2ff0(local_24);
  FUN_0044b1d0(iVar10 + 10,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8683);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 0xb,(int)local_48,pvVar2);
  iVar9 = FUN_004ed1c0((int)this_00);
  if (iVar9 == 0) {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8755);
  }
  else {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8756);
  }
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 0xb,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8681);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 0xc,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  iVar9 = FUN_004ed200((int)this_00);
  if (iVar9 == 0) {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8755);
  }
  else {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8756);
  }
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 0xc,(int)local_48,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8682);
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 0xd,(int)local_48,pvVar2);
  FUN_005f3c20(local_48);
  iVar9 = FUN_004ed1e0((int)this_00);
  if (iVar9 == 0) {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8755);
  }
  else {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8756);
  }
  FUN_005f3010(local_48,&param_1);
  FUN_0044b1d0(iVar10 + 0xd,(int)local_48,param_2);
  local_4._0_1_ = 3;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_18);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_48);
  ExceptionList = pvStack_c;
  return;
}

