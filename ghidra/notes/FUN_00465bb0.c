
void FUN_00465bb0(int *param_1)

{
  short sVar1;
  bool bVar2;
  bool bVar3;
  ushort uVar4;
  int *this;
  int *piVar5;
  int iVar6;
  uint uVar7;
  undefined *puVar8;
  char *pcVar9;
  uint unaff_EBX;
  undefined4 *puVar10;
  undefined2 local_44;
  ushort uStack_42;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_28;
  undefined4 local_24 [2];
  undefined4 uStack_1c;
  undefined4 local_18 [2];
  void *pvStack_10;
  void *pvStack_c;
  int iStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  iStack_8._0_1_ = 0xf8;
  iStack_8._1_3_ = 0x6318;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_24);
  local_4 = 0;
  FUN_004ece30((undefined4 *)&local_44);
  local_4._0_1_ = 1;
  FUN_005f2f50(local_18);
  this = param_1;
  local_4 = CONCAT31(local_4._1_3_,2);
  iVar6 = (**(code **)(*param_1 + 0xc))(&local_44);
  uVar4 = DAT_0065d424;
  if (iVar6 == 0) goto LAB_0046606f;
  FUN_005f3010(&uStack_28,&stack0x00000000);
  bVar2 = true;
  param_1[0x1b] = 1;
  if ((*(byte *)(this + 0x14) & 8) != 0) {
    FUN_005f3c20(&uStack_28);
    if (((this[0x10] & 0xffU) < 0x14) || (0x16 < (this[0x10] & 0xffU))) {
      FUN_005f3010(&uStack_28,&stack0x00000000);
    }
    else {
      FUN_005f3010(&uStack_28,&stack0x00000000);
    }
    goto LAB_0046606f;
  }
  iVar6 = FUN_004ece60((uint *)&stack0xffffffb8);
  if (iVar6 == 0) goto LAB_0046606f;
  uStack_40 = 0x90;
  uStack_3c = 0x98;
  uVar7 = unaff_EBX >> 0x18;
  iStack_8 = CONCAT31(iStack_8._1_3_,3);
  if ((uVar7 < 0x90) || (0x97 < uVar7)) {
    uStack_34 = 0x14;
    uStack_30 = 0x1c;
    if ((uVar7 < 0x14) || (0x1b < uVar7)) {
      bVar2 = false;
    }
    FUN_00619730();
    if (bVar2) goto LAB_00465d10;
    bVar2 = false;
  }
  else {
LAB_00465d10:
    bVar2 = true;
  }
  iStack_8._0_1_ = 2;
  FUN_00619730();
  if (!bVar2) {
    uStack_34 = 0x98;
    uStack_30 = 0xa0;
    if ((unaff_EBX >> 0x18 < 0x98) || (0x9f < unaff_EBX >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      FUN_005f3c20(&uStack_28);
      puVar8 = FUN_004f62d0(this[7]);
      pcVar9 = (char *)FUN_00583c40((int)puVar8);
      FUN_005f35b0(&uStack_34,pcVar9);
      iStack_8._0_1_ = 10;
      FUN_005f30d0(&uStack_28,(int)&uStack_34);
      iStack_8._0_1_ = 2;
      FUN_005f2ff0(&uStack_34);
      param_1[0x1b] = 1;
    }
    goto LAB_0046606f;
  }
  FUN_0042d170(this,&stack0x00000000);
  iStack_8._0_1_ = 4;
  FUN_005f3c20(&uStack_28);
  puVar8 = FUN_004f62d0(this[7]);
  pcVar9 = (char *)FUN_00583c40((int)puVar8);
  FUN_005f35b0(&uStack_34,pcVar9);
  iStack_8._0_1_ = 5;
  FUN_005f30d0(&uStack_28,(int)&uStack_34);
  iStack_8 = CONCAT31(iStack_8._1_3_,4);
  FUN_005f2ff0(&uStack_34);
  bVar2 = true;
  if ((*(byte *)(this + 0x2b) & 1) == 0) {
    iVar6 = FUN_004ece60((uint *)&stack0x00000000);
    if (iVar6 != 0) {
      uStack_34 = 0x50;
      uStack_30 = 0x80;
      if ((uVar4 >> 8 < 0x50) || (0x7f < uVar4 >> 8)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      FUN_00619730();
      if (bVar3) {
        local_44 = 0x8745;
        goto LAB_00465e01;
      }
    }
  }
  else {
    local_44 = 0x8746;
LAB_00465e01:
    uStack_42 = DAT_0065d424;
    FUN_005f3010(&uStack_1c,&local_44);
    bVar2 = false;
  }
  if (((short)this[0x25] != 0) && (bVar2)) {
    local_44 = 0x8747;
    uStack_42 = DAT_0065d424;
    FUN_005f3010(&uStack_1c,&local_44);
    bVar2 = false;
  }
  if (((*(byte *)(this + 0x14) & 0x10) != 0) && (bVar2)) {
    local_44 = 0x8744;
    uStack_42 = DAT_0065d424;
    FUN_005f3010(&uStack_1c,&local_44);
  }
  uVar7 = FUN_005f3070((int)&uStack_1c);
  if ((short)uVar7 != 0) {
    FUN_005f35b0(&uStack_34,&DAT_006a8790);
    iStack_8._0_1_ = 6;
    FUN_005f30d0(&uStack_28,(int)&uStack_34);
    iStack_8._0_1_ = 4;
    FUN_005f2ff0(&uStack_34);
    FUN_005f30d0(&uStack_28,(int)&uStack_1c);
    FUN_005f35b0(&uStack_34,&DAT_006a878c);
    iStack_8._0_1_ = 7;
    FUN_005f30d0(&uStack_28,(int)&uStack_34);
    iStack_8 = CONCAT31(iStack_8._1_3_,4);
    FUN_005f2ff0(&uStack_34);
  }
  if (*(short *)((int)this + 0x96) != 0) {
    FUN_005f3c20(&uStack_1c);
    sVar1 = *(short *)((int)this + 0x96);
    if (sVar1 == 1) {
      puVar10 = &uStack_34;
      uStack_34 = CONCAT22(DAT_0065d424,0x8802);
LAB_00465f4e:
      FUN_005f3010(&uStack_1c,puVar10);
    }
    else {
      if (sVar1 == 2) {
        puVar10 = &uStack_40;
        uStack_40 = CONCAT22(DAT_0065d424,0x8801);
        goto LAB_00465f4e;
      }
      if (sVar1 == 3) {
        local_44 = 0x8800;
        uStack_42 = DAT_0065d424;
        puVar10 = (undefined4 *)&local_44;
        goto LAB_00465f4e;
      }
    }
    uVar7 = FUN_005f3070((int)&uStack_1c);
    if ((short)uVar7 != 0) {
      FUN_005f35b0(&uStack_34,&DAT_006a8790);
      iStack_8._0_1_ = 8;
      FUN_005f30d0(&uStack_28,(int)&uStack_34);
      iStack_8._0_1_ = 4;
      FUN_005f2ff0(&uStack_34);
      FUN_005f30d0(&uStack_28,(int)&uStack_1c);
      FUN_005f35b0(&uStack_34,&DAT_006a878c);
      iStack_8._0_1_ = 9;
      FUN_005f30d0(&uStack_28,(int)&uStack_34);
      iStack_8 = CONCAT31(iStack_8._1_3_,4);
      FUN_005f2ff0(&uStack_34);
    }
  }
  iStack_8._0_1_ = 2;
  param_1[0x1b] = 1;
  FUN_00619730();
LAB_0046606f:
  piVar5 = param_1;
  if (param_1[0x1b] != 0) {
    FUN_005f2f50(&uStack_34);
    iStack_8._0_1_ = 0xb;
    FUN_005f2f50(&uStack_40);
    iStack_8._0_1_ = 0xc;
    pcVar9 = (char *)FUN_00583c40(this[0xb] + 0x34);
    FUN_005f35e0(&uStack_34,pcVar9);
    param_1 = (int *)CONCAT22(DAT_0065d424,0x1897);
    FUN_005f3010(&uStack_40,&param_1);
    FUN_005f30d0(&uStack_34,(int)&uStack_40);
    FUN_005f30d0(&uStack_34,(int)&uStack_28);
    pcVar9 = (char *)FUN_00583c40((int)&uStack_34);
    FUN_005f35e0(piVar5 + 5,pcVar9);
    iStack_8._0_1_ = 0xb;
    FUN_005f2ff0(&uStack_40);
    iStack_8._0_1_ = 2;
    FUN_005f2ff0(&uStack_34);
  }
  iStack_8._0_1_ = 1;
  FUN_005f2ff0(&uStack_1c);
  iStack_8 = (uint)iStack_8._1_3_ << 8;
  FUN_00619730();
  iStack_8 = 0xffffffff;
  FUN_005f2ff0(&uStack_28);
  ExceptionList = pvStack_10;
  return;
}

