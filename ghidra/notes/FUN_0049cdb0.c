
void FUN_0049cdb0(undefined4 *param_1)

{
  bool bVar1;
  bool bVar2;
  int *piVar3;
  uint *puVar4;
  uint uVar5;
  uint local_64;
  int *local_60;
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
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006370d0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_64);
  bVar2 = false;
  local_4 = 0;
  FUN_004f26d0(&local_64,param_1);
  do {
    local_5c = 0x90;
    uVar5 = local_64 >> 0x18;
    local_58 = 0x98;
    if ((uVar5 < 0x90) || (0x97 < uVar5)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_004c5d20(local_60 + 2,&local_64);
LAB_0049ce3d:
      bVar2 = true;
    }
    else {
      local_54 = 0x14;
      local_50 = 0x1c;
      if ((uVar5 < 0x14) || (0x1b < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0049ce3d;
      local_4c = 0x80;
      local_48 = 0x90;
      if ((uVar5 < 0x80) || (0x8f < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_004c5850(local_60 + 8,&local_64);
        goto LAB_0049ce3d;
      }
      local_44 = 8;
      local_40 = 0x10;
      if ((uVar5 < 8) || (0xf < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0049ce3d;
      local_3c = 0x30;
      local_38 = 0x40;
      if ((uVar5 < 0x30) || (0x3f < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0049ce3d;
      local_34 = 0x40;
      local_30 = 0x80;
      if ((uVar5 < 0x40) || (0x7f < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0049ce3d;
      local_2c = 0xf2;
      local_28 = 0xf3;
      if ((uVar5 < 0xf2) || (0xf2 < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0049ce3d;
      local_24 = 0xf8;
      local_20 = 0xf9;
      if ((uVar5 < 0xf8) || (0xf8 < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0049ce3d;
      local_1c = 0xf1;
      local_18 = 0xf2;
      if ((uVar5 < 0xf1) || (0xf1 < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0049ce3d;
      local_14 = 0x98;
      local_10 = 0xa0;
      if ((uVar5 < 0x98) || (0x9f < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0049ce3d;
      piVar3 = FUN_004f2d10(*local_60,&local_64);
      if ((piVar3 == (int *)0x0) || ((void *)piVar3[7] == (void *)0x0)) goto LAB_0049ce3d;
      puVar4 = FUN_004025b0((void *)piVar3[7],(uint *)&param_1);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_64,puVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    if (bVar2) {
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = local_c;
      return;
    }
  } while( true );
}

