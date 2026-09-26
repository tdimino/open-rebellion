
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void * __thiscall
FUN_005d26c0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,short param_4)

{
  int *piVar1;
  float *pfVar2;
  float fVar3;
  int iVar4;
  undefined4 uVar5;
  uint uVar6;
  undefined4 *puVar7;
  void *unaff_EBX;
  int *piStack_118;
  undefined4 local_b8;
  undefined4 local_b4;
  int local_b0;
  undefined4 local_ac;
  undefined4 local_a8;
  undefined4 local_a4;
  undefined4 local_a0;
  undefined4 local_9c;
  undefined4 local_98;
  undefined4 local_94;
  undefined4 local_90;
  undefined4 local_8c;
  undefined4 local_88;
  undefined4 local_84;
  undefined4 local_80;
  undefined4 local_7c;
  undefined4 local_78;
  undefined4 local_74;
  undefined4 local_70;
  undefined4 local_6c;
  undefined4 local_68;
  undefined4 local_64;
  undefined4 local_60;
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c;
  undefined4 local_48;
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654654;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = this;
  FUN_005d8980(this,param_2,param_1);
  local_4 = 0;
  FUN_005fbb20((undefined4 *)((int)this + 0xf0));
  local_4._0_1_ = 1;
  FUN_005fbb20((undefined4 *)((int)this + 0x108));
  *(undefined4 *)((int)this + 0x68) = 0x46;
  *(undefined4 *)((int)this + 100) = 0x46;
  *(undefined4 *)((int)this + 0x3c) = param_3;
  *(undefined4 *)((int)this + 0xe8) = 0x3f800000;
  *(undefined4 *)((int)this + 0xe4) = 0x3f800000;
  *(undefined ***)this = &PTR_FUN_0066d198;
  *(undefined ***)((int)this + 0x1c) = &PTR_LAB_0066d188;
  local_4 = CONCAT31(local_4._1_3_,2);
  *(undefined4 *)((int)this + 0x134) = 0;
  *(undefined4 *)((int)this + 0x144) = 0;
  *(undefined4 *)((int)this + 300) = 0;
  *(undefined4 *)((int)this + 0x40) = 1;
  *(undefined4 *)((int)this + 0x60) = 0;
  *(undefined4 *)((int)this + 0x5c) = 0;
  *(undefined4 *)((int)this + 0x120) = 0;
  *(undefined4 *)((int)this + 0x44) = 0;
  *(undefined4 *)((int)this + 0x48) = 0;
  *(undefined4 *)((int)this + 0x4c) = 0;
  *(undefined4 *)((int)this + 0x130) = 0;
  *(undefined4 *)((int)this + 0x140) = 0;
  local_b8 = 0xbf000000;
  local_b4 = 0x3f000000;
  local_b0 = -0x41000000;
  local_ac = 0x3f000000;
  local_a8 = 0x3f000000;
  local_a4 = 0x3f000000;
  local_a0 = 0x3f000000;
  local_9c = 0x3f000000;
  local_98 = 0x3f000000;
  local_94 = 0xbf000000;
  local_90 = 4;
  local_8c = 0;
  local_88 = 3;
  local_84 = 2;
  local_80 = 1;
  local_7c = 4;
  local_78 = 0;
  local_74 = 4;
  local_6c = 3;
  local_60 = 1;
  local_34 = 1;
  local_70 = 7;
  local_68 = 4;
  local_64 = 0;
  local_5c = 5;
  local_58 = 4;
  local_54 = 4;
  local_50 = 6;
  local_4c = 7;
  local_48 = 4;
  local_44 = 5;
  local_40 = 4;
  local_3c = 6;
  local_38 = 5;
  local_30 = 2;
  local_2c = 4;
  local_28 = 6;
  local_24 = 2;
  local_20 = 3;
  local_1c = 7;
  local_18 = 0;
  *(undefined4 *)((int)this + 0x138) = 0;
  *(undefined4 *)((int)this + 0x124) = 0;
  *(undefined4 *)((int)this + 0x148) = 0;
  *(undefined4 *)((int)this + 0x128) = 0;
  FUN_005fefd0(6);
  piVar1 = (int *)(DAT_006bcbd4 + 0x198);
  for (iVar4 = FUN_005f5060(DAT_006bcbd4 + 0x19c); iVar4 != 0; iVar4 = *(int *)(iVar4 + 0x10)) {
    if (*(short *)(iVar4 + 0x18) == param_4) goto LAB_005d29a8;
  }
  iVar4 = 0;
LAB_005d29a8:
  if (iVar4 == 0) {
    uVar5 = (**(code **)(*piVar1 + 4))();
    *(undefined4 *)((int)this + 0x4c) = uVar5;
  }
  else {
    *(undefined4 *)((int)this + 0x4c) = *(undefined4 *)(iVar4 + 0x1c);
  }
  iVar4 = (**(code **)(**(int **)((int)this + 0x4c) + 0x18))();
  piStack_118 = (int *)0x5d29e8;
  FUN_005d8b00(iVar4);
  piVar1 = (int *)(DAT_006bcbd4 + 0x198);
  for (iVar4 = FUN_005f5060(DAT_006bcbd4 + 0x19c); iVar4 != 0; iVar4 = *(int *)(iVar4 + 0x10)) {
    if (*(short *)(iVar4 + 0x18) == (short)(param_4 + 1)) goto LAB_005d2a1c;
  }
  iVar4 = 0;
LAB_005d2a1c:
  if (iVar4 == 0) {
    uVar5 = (**(code **)(*piVar1 + 4))();
    *(undefined4 *)((int)this + 0x48) = uVar5;
  }
  else {
    *(undefined4 *)((int)this + 0x48) = *(undefined4 *)(iVar4 + 0x1c);
  }
  piStack_118 = *(int **)((int)this + 0x48);
  iVar4 = (**(code **)(*piStack_118 + 0x18))();
  FUN_005d8b00(iVar4);
  piVar1 = (int *)(DAT_006bcbd4 + 0x198);
  piStack_118 = (int *)0x5d2a7a;
  for (iVar4 = FUN_005f5060(DAT_006bcbd4 + 0x19c); iVar4 != 0; iVar4 = *(int *)(iVar4 + 0x10)) {
    if (*(short *)(iVar4 + 0x18) == (short)(param_4 + 2)) goto LAB_005d2a91;
  }
  iVar4 = 0;
LAB_005d2a91:
  if (iVar4 == 0) {
    piStack_118 = (int *)&stack0xfffffefc;
    uVar5 = (**(code **)(*piVar1 + 4))();
    *(undefined4 *)((int)this + 0x44) = uVar5;
  }
  else {
    *(undefined4 *)((int)this + 0x44) = *(undefined4 *)(iVar4 + 0x1c);
  }
  piStack_118 = (int *)0x0;
  iVar4 = (**(code **)(**(int **)((int)this + 0x44) + 0x18))
                    (*(int **)((int)this + 0x44),
                     (-(uint)(*(int *)((int)this + 0x3c) != 1) & 0xffffffc0) + 0x80 | 0x11);
  FUN_005d8b00(iVar4);
  pfVar2 = (float *)((int)this + 0xcc);
  iVar4 = (**(code **)(**(int **)((int)this + 0x48) + 0x40))(*(int **)((int)this + 0x48),pfVar2,0);
  FUN_005d8b00(iVar4);
  uVar6 = 0;
  puVar7 = (undefined4 *)((int)this + 0x70);
  do {
    if ((uVar6 & 4) == 0) {
      fVar3 = *pfVar2;
    }
    else {
      fVar3 = *(float *)((int)this + 0xd8);
    }
    puVar7[-1] = fVar3;
    if ((uVar6 & 2) == 0) {
      uVar5 = *(undefined4 *)((int)this + 0xd0);
    }
    else {
      uVar5 = *(undefined4 *)((int)this + 0xdc);
    }
    *puVar7 = uVar5;
    if ((uVar6 & 1) == 0) {
      uVar5 = *(undefined4 *)((int)this + 0xd4);
    }
    else {
      uVar5 = *(undefined4 *)((int)this + 0xe0);
    }
    puVar7[1] = uVar5;
    uVar6 = uVar6 + 1;
    puVar7 = puVar7 + 3;
  } while ((int)uVar6 < 8);
  puVar7 = (undefined4 *)((int)this + 0x50);
  iVar4 = (**(code **)(*DAT_006bcbd0 + 0x18))(DAT_006bcbd0,puVar7,0);
  FUN_005d8b00(iVar4);
  iVar4 = (**(code **)(*(int *)*puVar7 + 0x5c))
                    ((int *)*puVar7,8,&piStack_118,0,0,&local_b8,&local_3c,0);
  FUN_005d8b00(iVar4);
  iVar4 = (**(code **)(*(int *)*puVar7 + 0x78))((int *)*puVar7,0x40,0);
  FUN_005d8b00(iVar4);
  iVar4 = (**(code **)(**(int **)((int)this + 0x48) + 0x40))(*(int **)((int)this + 0x48),pfVar2,0);
  FUN_005d8b00(iVar4);
  iVar4 = (**(code **)(*(int *)*puVar7 + 0x34))
                    ((int *)*puVar7,*(float *)((int)this + 0xd8) - *pfVar2,
                     *(float *)((int)this + 0xdc) - *(float *)((int)this + 0xd0),
                     *(float *)((int)this + 0xe0) - *(float *)((int)this + 0xd4),0);
  FUN_005d8b00(iVar4);
  iVar4 = (**(code **)(*(int *)*puVar7 + 0x38))
                    ((int *)*puVar7,(*(float *)((int)this + 0xd8) + *pfVar2) * _DAT_0066d174,
                     (*(float *)((int)this + 0xd0) + *(float *)((int)this + 0xdc)) * _DAT_0066d174,
                     (*(float *)((int)this + 0xd4) + *(float *)((int)this + 0xe0)) * _DAT_0066d174,0
                    );
  FUN_005d8b00(iVar4);
  iVar4 = (**(code **)(*(int *)*puVar7 + 0xc))
                    ((int *)*puVar7,0,&DAT_0066e428,(undefined4 *)((int)this + 0x54),0);
  FUN_005d8b00(iVar4);
  iVar4 = (**(code **)(*(int *)*puVar7 + 100))((int *)*puVar7,0x3f800000,0x3f800000,0,0);
  FUN_005d8b00(iVar4);
  piVar1 = *(int **)((int)this + 0x54);
  if (*(int *)((int)this + 0x3c) == 1) {
    iVar4 = (**(code **)(*piVar1 + 100))(piVar1,0,0x3f800000,0);
  }
  else {
    iVar4 = (**(code **)(*piVar1 + 100))(piVar1,0x3f800000,0,0,0);
  }
  FUN_005d8b00(iVar4);
  iVar4 = (**(code **)(*(int *)*puVar7 + 0xc))
                    ((int *)*puVar7,0,&DAT_0066e428,(undefined4 *)((int)this + 0x58),0);
  FUN_005d8b00(iVar4);
  piVar1 = *(int **)((int)this + 0x58);
  iVar4 = (**(code **)(*piVar1 + 100))(piVar1,0x3f800000,0x3f800000,0x3f800000,0);
  FUN_005d8b00(iVar4);
  *(undefined4 *)((int)this + 0x38) = 0;
  if (local_b0 == 1) {
    FUN_005d31f0(this);
  }
  else if (local_b0 == 2) {
    FUN_005d3360(this);
  }
  else if (local_b0 == 3) {
    FUN_005d3300(this);
  }
  else {
    FUN_005d3180(this);
  }
  *(undefined4 *)((int)this + 0x120) = 0;
  iVar4 = (**(code **)(*DAT_006bcbd0 + 0x10))
                    (DAT_006bcbd0,*(undefined4 *)((int)this + 0x2c),
                     (undefined4 *)((int)this + 0x124),0);
  FUN_005d8b00(iVar4);
  piVar1 = *(int **)((int)this + 0x124);
  (**(code **)(*piVar1 + 0x18))(piVar1,0);
  *(undefined4 *)((int)this + 0x138) = 0;
  iVar4 = (**(code **)(*DAT_006bcbd0 + 0x10))
                    (DAT_006bcbd0,*(undefined4 *)((int)this + 0x2c),
                     (undefined4 *)((int)this + 0x128),0);
  FUN_005d8b00(iVar4);
  piVar1 = *(int **)((int)this + 0x128);
  (**(code **)(*piVar1 + 0x18))(piVar1,0);
  *(undefined4 *)((int)this + 0x148) = 0;
  ExceptionList = unaff_EBX;
  return this;
}

