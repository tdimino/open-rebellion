
void __thiscall FUN_00449200(void *this,void *param_1,void *param_2)

{
  uint uVar1;
  char *pcVar2;
  undefined *puVar3;
  int iVar4;
  int *piVar5;
  int iVar6;
  undefined4 local_a8;
  undefined4 local_a4 [2];
  undefined2 uStack_9c;
  undefined2 uStack_9a;
  undefined4 local_98;
  undefined4 local_94 [3];
  undefined4 local_88;
  void *local_84;
  int local_80;
  int local_7c;
  int local_78;
  int local_74;
  int local_70;
  int local_6c;
  int local_68;
  undefined4 local_64 [3];
  undefined4 local_58 [2];
  undefined4 uStack_50;
  undefined1 local_4c [28];
  int local_30;
  undefined4 auStack_2c [7];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f24a;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_a4);
  local_4 = 0;
  local_a8 = CONCAT22(DAT_0065d424,0x8738);
  FUN_005f3010(local_a4,&local_a8);
  pcVar2 = (char *)FUN_00583c40((int)local_a4);
  FUN_00601aa0((void *)((int)this + 0x130),pcVar2);
  local_84 = *(void **)((int)this + 300);
  FUN_0044a210((int)this);
  local_a8 = CONCAT22(DAT_0065d424,0x8666);
  FUN_005f3010(local_a4,&local_a8);
  iVar6 = 2;
  FUN_0044b1d0(2,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  uVar1 = *(uint *)(*(int *)((int)this + 300) + 0x50);
  if ((uVar1 & 0x10) == 0) {
    if ((uVar1 & 4) == 0) {
      local_a8 = CONCAT22(DAT_0065d424,0x8757);
      goto LAB_00449339;
    }
    if ((uVar1 & 0x20) != 0) {
      local_a8 = CONCAT22(DAT_0065d424,0x8744);
      goto LAB_00449339;
    }
    local_a8._0_2_ = 0x8748;
  }
  else {
    local_a8._0_2_ = 0x8744;
  }
  local_a8 = CONCAT22(DAT_0065d424,(undefined2)local_a8);
LAB_00449339:
  FUN_005f3010(local_a4,&local_a8);
  FUN_0044b1d0(2,(int)local_a4,param_2);
  local_6c = 1;
  local_68 = -1;
  local_4._0_1_ = 1;
  local_70 = 0;
  FUN_004fd2b0(*(int **)((int)this + 300),&local_6c,&local_70);
  if ((local_68 == -1) && (local_70 != 0)) {
    local_88._0_2_ = 0x3814;
    local_88._2_2_ = DAT_0065d424;
    FUN_005f2fc0(local_94,&local_88);
    local_4._0_1_ = 2;
    FUN_005f3c20(local_a4);
    local_88 = CONCAT22(local_88._2_2_,0x8766);
    FUN_005f2fc0(local_58,&local_88);
    local_4._0_1_ = 3;
    pcVar2 = (char *)FUN_00583c40((int)local_58);
    FUN_005f35b0(local_64,pcVar2);
    local_4._0_1_ = 4;
    FUN_005f30d0(local_a4,(int)local_64);
    local_4._0_1_ = 3;
    FUN_005f2ff0(local_64);
    iVar6 = 3;
    FUN_0044b1d0(3,(int)local_a4,param_1);
    FUN_005f3c20(local_a4);
    pcVar2 = (char *)FUN_00583c40((int)local_94);
    FUN_005f35b0(local_64,pcVar2);
    local_4._0_1_ = 5;
    FUN_005f30d0(local_a4,(int)local_64);
    FUN_005f31a0(local_a4,local_70);
    local_4._0_1_ = 3;
    FUN_005f2ff0(local_64);
    FUN_0044b1d0(3,(int)local_a4,param_2);
    local_4._0_1_ = 2;
    FUN_005f2ff0(local_58);
    local_4._0_1_ = 1;
    FUN_005f2ff0(local_94);
  }
  FUN_005f3c20(local_a4);
  local_a8 = CONCAT22(DAT_0065d424,0x3046);
  FUN_005f3010(local_a4,&local_a8);
  FUN_005f35b0(local_94,&DAT_006a872c);
  local_4._0_1_ = 6;
  FUN_005f30d0(local_a4,(int)local_94);
  local_4._0_1_ = 1;
  FUN_005f2ff0(local_94);
  FUN_0044b1d0(iVar6 + 1,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  FUN_004fd790(local_84,2,&local_80);
  if (local_80 == 0) {
    local_a8 = CONCAT22(DAT_0065d424,0x8762);
    FUN_005f3010(local_a4,&local_a8);
  }
  else {
    puVar3 = FUN_004f62d0(local_80);
    pcVar2 = (char *)FUN_00583c40((int)puVar3);
    FUN_005f35b0(local_94,pcVar2);
    local_4._0_1_ = 7;
    FUN_005f30d0(local_a4,(int)local_94);
    local_4._0_1_ = 1;
    FUN_005f2ff0(local_94);
  }
  FUN_0044b1d0(iVar6 + 1,(int)local_a4,param_2);
  FUN_005f3c20(local_a4);
  local_a8 = CONCAT22(DAT_0065d424,0x3048);
  FUN_005f3010(local_a4,&local_a8);
  FUN_005f35b0(local_94,&DAT_006a872c);
  local_4._0_1_ = 8;
  FUN_005f30d0(local_a4,(int)local_94);
  local_4._0_1_ = 1;
  FUN_005f2ff0(local_94);
  FUN_0044b1d0(iVar6 + 2,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  FUN_004fd790(local_84,3,&local_80);
  if (local_80 == 0) {
    local_a8 = CONCAT22(DAT_0065d424,0x8762);
    FUN_005f3010(local_a4,&local_a8);
  }
  else {
    puVar3 = FUN_004f62d0(local_80);
    pcVar2 = (char *)FUN_00583c40((int)puVar3);
    FUN_005f35b0(local_94,pcVar2);
    local_4._0_1_ = 9;
    FUN_005f30d0(local_a4,(int)local_94);
    local_4._0_1_ = 1;
    FUN_005f2ff0(local_94);
  }
  FUN_0044b1d0(iVar6 + 2,(int)local_a4,param_2);
  FUN_005f3c20(local_a4);
  local_a8 = CONCAT22(DAT_0065d424,0x3044);
  FUN_005f3010(local_a4,&local_a8);
  FUN_005f35b0(local_94,&DAT_006a872c);
  local_4._0_1_ = 10;
  FUN_005f30d0(local_a4,(int)local_94);
  local_4._0_1_ = 1;
  FUN_005f2ff0(local_94);
  FUN_0044b1d0(iVar6 + 3,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  FUN_004fd790(local_84,1,&local_80);
  if (local_80 == 0) {
    local_a8 = CONCAT22(DAT_0065d424,0x8762);
    FUN_005f3010(local_a4,&local_a8);
  }
  else {
    puVar3 = FUN_004f62d0(local_80);
    pcVar2 = (char *)FUN_00583c40((int)puVar3);
    FUN_005f35b0(local_94,pcVar2);
    local_4._0_1_ = 0xb;
    FUN_005f30d0(local_a4,(int)local_94);
    local_4._0_1_ = 1;
    FUN_005f2ff0(local_94);
  }
  FUN_0044b1d0(iVar6 + 3,(int)local_a4,param_2);
  local_a8 = CONCAT22(DAT_0065d424,0x8740);
  FUN_005f3010(local_a4,&local_a8);
  FUN_0044b1d0(iVar6 + 4,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  iVar4 = FUN_00501e70(*(void **)((int)this + 300),
                       *(uint *)((int)*(void **)((int)this + 300) + 0x24) >> 6 & 3,3);
  FUN_005f31a0(local_a4,iVar4);
  FUN_0044b1d0(iVar6 + 4,(int)local_a4,param_2);
  local_a8 = 0;
  local_84 = (void *)0x0;
  local_88 = 0;
  local_74 = 0;
  local_78 = 0;
  local_7c = 0;
  FUN_00502db0(local_4c,*(undefined4 *)((int)this + 300),3);
  local_4 = CONCAT31(local_4._1_3_,0xc);
  FUN_00513120((int)local_4c);
  while (local_30 != 0) {
    piVar5 = (int *)FUN_0052bed0((int)local_4c);
    if ((piVar5[0x14] & 0x200U) != 0) {
      local_a8 = local_a8 + 1;
    }
    iVar4 = (**(code **)(*piVar5 + 0x26c))();
    local_84 = (void *)((int)local_84 + iVar4);
    iVar4 = (**(code **)(*piVar5 + 0x270))();
    local_88 = local_88 + iVar4;
    iVar4 = (**(code **)(*piVar5 + 0x23c))();
    local_74 = local_74 + iVar4;
    iVar4 = (**(code **)(*piVar5 + 0x240))();
    local_78 = local_78 + iVar4;
    FUN_00536da0(auStack_2c,piVar5,3);
    local_4._0_1_ = 0xd;
    iVar4 = FUN_00513180((int)auStack_2c);
    local_4 = CONCAT31(local_4._1_3_,0xc);
    local_7c = local_7c + iVar4;
    FUN_00536ea0(auStack_2c);
    FUN_005130d0((int)local_4c);
  }
  local_98._0_2_ = 0x8704;
  local_98._2_2_ = DAT_0065d424;
  FUN_005f3010(local_a4,&local_98);
  FUN_0044b1d0(iVar6 + 5,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  FUN_005f35b0(local_94,&DAT_006b120c);
  local_4._0_1_ = 0xe;
  FUN_005f30d0(local_a4,(int)local_94);
  local_4._0_1_ = 0xc;
  FUN_005f2ff0(local_94);
  FUN_0044b1d0(iVar6 + 5,(int)local_a4,param_2);
  FUN_005f3c20(local_a4);
  local_98._0_2_ = 0x3820;
  local_98._2_2_ = DAT_0065d424;
  FUN_005f3010(local_a4,&local_98);
  FUN_005f35b0(local_94,&DAT_006a872c);
  local_4._0_1_ = 0xf;
  FUN_005f30d0(local_a4,(int)local_94);
  local_4._0_1_ = 0xc;
  FUN_005f2ff0(local_94);
  FUN_0044b1d0(iVar6 + 6,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  FUN_005f31a0(local_a4,local_84);
  FUN_0044b1d0(iVar6 + 6,(int)local_a4,param_2);
  local_98._0_2_ = 0x8705;
  local_98._2_2_ = DAT_0065d424;
  FUN_005f3010(local_a4,&local_98);
  FUN_0044b1d0(iVar6 + 7,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  FUN_005f31a0(local_a4,local_88);
  FUN_0044b1d0(iVar6 + 7,(int)local_a4,param_2);
  local_98._0_2_ = 0x8706;
  local_98._2_2_ = DAT_0065d424;
  FUN_005f3010(local_a4,&local_98);
  FUN_0044b1d0(iVar6 + 8,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  FUN_005f35b0(local_94,&DAT_006b120c);
  local_4._0_1_ = 0x10;
  FUN_005f30d0(local_a4,(int)local_94);
  local_4._0_1_ = 0xc;
  FUN_005f2ff0(local_94);
  FUN_0044b1d0(iVar6 + 8,(int)local_a4,param_2);
  FUN_005f3c20(local_a4);
  local_98._2_2_ = DAT_0065d424;
  local_98._0_2_ = 0x3820;
  FUN_005f3010(local_a4,&local_98);
  FUN_005f35b0(local_94,&DAT_006a872c);
  local_4._0_1_ = 0x11;
  FUN_005f30d0(local_a4,(int)local_94);
  local_4 = CONCAT31(local_4._1_3_,0xc);
  FUN_005f2ff0(local_94);
  FUN_0044b1d0(iVar6 + 9,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  FUN_005f31a0(local_a4,local_74);
  FUN_0044b1d0(iVar6 + 9,(int)local_a4,param_2);
  local_98._0_2_ = 0x8705;
  local_98._2_2_ = DAT_0065d424;
  FUN_005f3010(local_a4,&local_98);
  FUN_0044b1d0(iVar6 + 10,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  FUN_005f31a0(local_a4,local_78);
  FUN_0044b1d0(iVar6 + 10,(int)local_a4,param_2);
  local_98._2_2_ = DAT_0065d424;
  local_98._0_2_ = 0x8707;
  FUN_005f3010(local_a4,&local_98);
  FUN_0044b1d0(iVar6 + 0xb,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  FUN_005f31a0(local_a4,local_7c);
  FUN_0044b1d0(iVar6 + 0xb,(int)local_a4,param_2);
  local_98._0_2_ = 0x8741;
  local_98._2_2_ = DAT_0065d424;
  FUN_005f3010(local_a4,&local_98);
  FUN_0044b1d0(iVar6 + 0xc,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  FUN_005f31a0(local_a4,local_a8);
  FUN_0044b1d0(iVar6 + 0xc,(int)local_a4,param_2);
  local_98._0_2_ = 0x8653;
  local_98._2_2_ = DAT_0065d424;
  FUN_005f3010(local_a4,&local_98);
  FUN_0044b1d0(iVar6 + 0xd,(int)local_a4,param_1);
  FUN_005f3c20(local_a4);
  iVar4 = FUN_004067d0();
  if (iVar4 == 0) {
    iVar4 = (**(code **)(**(int **)((int)this + 300) + 0x34))(1);
    if (iVar4 == 0) {
      uStack_9c = 0x8755;
    }
    else {
      uStack_9c = 0x8756;
    }
    uStack_9a = DAT_0065d424;
    FUN_005f3010(&local_a8,&uStack_9c);
  }
  else {
    iVar4 = (**(code **)(**(int **)((int)this + 300) + 0x34))();
    if (iVar4 == 0) {
      FUN_005f35b0(&local_98,&DAT_006a8730);
      puStack_8._0_1_ = 0x13;
      FUN_005f30d0(&local_a8,(int)&local_98);
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,0xc);
      FUN_005f2ff0(&local_98);
    }
    else {
      FUN_005f35b0(&local_98,&DAT_006a8734);
      puStack_8._0_1_ = 0x12;
      FUN_005f30d0(&local_a8,(int)&local_98);
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,0xc);
      FUN_005f2ff0(&local_98);
    }
  }
  FUN_0044b1d0(iVar6 + 0xd,(int)&local_a8,param_2);
  puStack_8._0_1_ = 1;
  FUN_00502eb0(&uStack_50);
  puStack_8 = (undefined1 *)((uint)puStack_8._1_3_ << 8);
  FUN_00619730();
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_005f2ff0(&local_a8);
  ExceptionList = pvStack_10;
  return;
}

