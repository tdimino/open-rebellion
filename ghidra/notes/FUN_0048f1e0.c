
void __thiscall FUN_0048f1e0(void *this,int *param_1,void *param_2)

{
  bool bVar1;
  void *this_00;
  ushort uVar2;
  undefined4 *puVar3;
  int *piVar4;
  int iVar5;
  uint uVar6;
  int *piVar7;
  int *piVar8;
  int *piVar9;
  uint local_30;
  uint local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635940;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(local_2c);
  local_4 = 0;
  FUN_004ece30(&local_30);
  this_00 = param_2;
  local_4._0_1_ = 1;
  if (param_1 != (int *)0x362) goto switchD_0048f4ef_caseD_244;
  FUN_004f26d0(local_2c,(undefined4 *)((int)param_2 + 0x20));
  puVar3 = (undefined4 *)FUN_00494ed0(this_00,&param_2);
  local_4._0_1_ = 2;
  FUN_004f26d0(&local_30,puVar3);
  local_4._0_1_ = 1;
  FUN_00619730();
  piVar4 = FUN_004f2d10(*(int *)((int)this + 0x20),local_2c);
  param_1 = FUN_004f2d10(*(int *)((int)this + 0x20),&local_30);
  *(ushort *)((int)this + 0x2e) = (*(int *)((int)this + 0x20) != 1) + 0x412;
  *(ushort *)((int)this + 0x30) = (*(int *)((int)this + 0x20) != 1) + 0x47c;
  if ((piVar4 != (int *)0x0) && (param_1 != (int *)0x0)) {
    FUN_004f26d0((void *)((int)this + 0x5c),&local_30);
    local_2c[4] = 0x30;
    local_2c[5] = 0x3c;
    local_4 = CONCAT31(local_4._1_3_,3);
    if ((local_2c[0] >> 0x18 < 0x30) || (0x3b < local_2c[0] >> 0x18)) {
LAB_0048f32b:
      bVar1 = false;
    }
    else {
      local_2c[2] = 0x30;
      local_2c[3] = 0x3c;
      if ((local_30 >> 0x18 < 0x30) || (0x3b < local_30 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) goto LAB_0048f32b;
      bVar1 = true;
    }
    local_4._0_1_ = 1;
    FUN_00619730();
    if (bVar1) {
      *(undefined4 *)((int)this + 0x40) = 1;
    }
    if (*(int *)((int)this + 0x20) == 1) {
      iVar5 = FUN_004edc80((int)piVar4);
      if (iVar5 != 1) goto LAB_0048f369;
    }
    else if ((*(int *)((int)this + 0x20) == 2) && (iVar5 = FUN_004edc80((int)piVar4), iVar5 != 2)) {
LAB_0048f369:
      *(undefined4 *)((int)this + 0x40) = 0;
    }
  }
  piVar9 = param_1;
  if (*(int *)((int)this + 0x40) != 0) {
    uVar6 = FUN_004c5000(piVar4);
    *(short *)((int)this + 0x2c) = (short)uVar6;
    local_2c[1] = 0x31000241;
    FUN_00619730();
    if (local_30 == 0x31000241) {
      FUN_004f26d0(local_2c,&local_30);
      uVar6 = FUN_004c5000(piVar9);
      *(short *)((int)this + 0x2c) = (short)uVar6;
      FUN_0060b9d0((void *)((int)this + 0x44),0x71d8,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
      piVar4 = (int *)0x0;
      piVar9 = (int *)0x0;
      uVar6 = 0x71d9;
    }
    else {
      if (piVar4 == (int *)0x0) {
        piVar8 = (int *)0x0;
      }
      else {
        piVar8 = piVar4 + 0xc;
      }
      if (piVar9 == (int *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = piVar9 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7178,piVar7,piVar8,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      FUN_004ed310(piVar4,&param_1);
      if (param_1 == (int *)0x0) {
        if (piVar4 == (int *)0x0) {
          piVar4 = (int *)0x0;
        }
        else {
          piVar4 = piVar4 + 0xc;
        }
        if (piVar9 == (int *)0x0) {
          piVar9 = (int *)0x0;
        }
        else {
          piVar9 = piVar9 + 0xc;
        }
        uVar6 = 0x717a;
      }
      else {
        if (piVar4 == (int *)0x0) {
          piVar4 = (int *)0x0;
        }
        else {
          piVar4 = piVar4 + 0xc;
        }
        if (piVar9 == (int *)0x0) {
          piVar9 = (int *)0x0;
          uVar6 = 0x7179;
        }
        else {
          piVar9 = piVar9 + 0xc;
          uVar6 = 0x7179;
        }
      }
    }
    FUN_0060b9d0((void *)((int)this + 0x50),uVar6,piVar9,piVar4,(undefined4 *)0x0,(undefined4 *)0x0)
    ;
    local_2c[6] = 0x30;
    local_2c[7] = 0x38;
    if ((local_2c[0] >> 0x18 < 0x30) || (0x37 < local_2c[0] >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      switch(local_2c[0] & 0xffffff) {
      case 0x240:
        *(undefined4 *)((int)this + 0x28) = 0x17;
        break;
      case 0x241:
        *(undefined4 *)((int)this + 0x28) = 0x18;
        uVar2 = FUN_004c4990(&DAT_00000124);
        *(ushort *)((int)this + 0x32) = uVar2;
        break;
      case 0x242:
        *(undefined4 *)((int)this + 0x28) = 0x16;
        uVar2 = FUN_004c4990(&DAT_00000135);
        *(ushort *)((int)this + 0x32) = uVar2;
        break;
      case 0x243:
        *(undefined4 *)((int)this + 0x28) = 0x19;
        break;
      case 0x280:
        *(undefined4 *)((int)this + 0x28) = 0x1a;
        break;
      case 0x281:
        *(undefined4 *)((int)this + 0x28) = 0x1b;
        uVar2 = FUN_004c4990(&DAT_00000136);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
    }
    else {
      *(undefined4 *)((int)this + 0x28) = 0x15;
    }
  }
switchD_0048f4ef_caseD_244:
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

