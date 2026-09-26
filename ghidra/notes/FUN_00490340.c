
void __thiscall FUN_00490340(void *this,int *param_1,int *param_2)

{
  ushort uVar1;
  int *piVar2;
  int *piVar3;
  uint *puVar4;
  int iVar5;
  int *piVar6;
  undefined4 *puVar7;
  bool bVar8;
  bool bVar9;
  uint uVar10;
  uint local_74;
  uint local_70;
  int local_6c;
  uint local_68;
  int local_64;
  undefined1 local_60 [4];
  undefined1 local_5c [4];
  undefined1 local_58 [4];
  uint local_54;
  undefined1 local_50 [4];
  undefined1 local_4c [4];
  undefined1 local_48 [4];
  uint local_44 [14];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  piVar6 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635b58;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_70);
  local_4 = 0;
  FUN_004ece30(&local_74);
  local_4._0_1_ = 1;
  if (param_1 != (int *)0x1e0) goto switchD_0049086b_caseD_244;
  FUN_004f26d0(&local_70,param_2 + 8);
  piVar2 = FUN_004f2d10(*(int *)((int)this + 0x20),&local_70);
  param_1 = (int *)0x0;
  param_2 = (int *)0x0;
  if (*(int *)((int)this + 0x20) == 1) {
    uVar10 = 2;
  }
  else {
    uVar10 = 1;
  }
  local_64 = FUN_004f3dd0(*(int *)((int)this + 0x20),uVar10);
  *(undefined2 *)((int)this + 0x30) = 0x44c;
  *(ushort *)((int)this + 0x2e) = (*(int *)((int)this + 0x20) != 1) + 0x412;
  if (piVar2 != (int *)0x0) {
    FUN_004ece30(&local_6c);
    local_4._0_1_ = 2;
    piVar3 = FUN_00490f60(piVar6,local_60);
    bVar8 = *piVar3 != local_6c;
    FUN_00619730();
    if (bVar8) {
      puVar4 = FUN_00490f60(piVar6,local_5c);
      local_4._0_1_ = 3;
      param_1 = FUN_004f2d10(*(int *)((int)this + 0x20),puVar4);
      local_4._0_1_ = 2;
      FUN_00619730();
    }
    if (param_1 == (int *)0x0) {
      piVar3 = FUN_0048a640(piVar6,local_58);
      local_54 = (uint)(*piVar3 != local_6c);
      FUN_00619730();
      if (local_54 != 0) {
        puVar4 = FUN_0048a640(piVar6,local_50);
        local_4._0_1_ = 4;
        param_1 = FUN_004f2d10(*(int *)((int)this + 0x20),puVar4);
        local_4._0_1_ = 2;
        FUN_00619730();
      }
      if (param_1 == (int *)0x0) {
        piVar3 = FUN_00490f40(piVar6,local_4c);
        iVar5 = *piVar3;
        FUN_00619730();
        if (iVar5 != local_6c) {
          puVar4 = FUN_00490f40(piVar6,local_48);
          local_4._0_1_ = 5;
          param_1 = FUN_004f2d10(*(int *)((int)this + 0x20),puVar4);
          local_4._0_1_ = 2;
          FUN_00619730();
        }
      }
    }
    local_44[2] = 0x30;
    local_44[3] = 0x3c;
    if ((local_70 >> 0x18 < 0x30) || (0x3b < local_70 >> 0x18)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    FUN_00619730();
    if (bVar8) {
      FUN_004f26d0((void *)((int)this + 0x5c),&local_70);
      *(undefined4 *)((int)this + 0x40) = 1;
      iVar5 = FUN_004edc80((int)piVar2);
      bVar8 = iVar5 == 1;
      if (*(int *)((int)this + 0x20) == 2) {
        bVar8 = !bVar8;
      }
      param_2 = piVar2;
      if ((short)piVar2[0x26] == 0) {
        local_68 = 0x98000481;
        local_4._0_1_ = 6;
        puVar4 = FUN_004025b0((void *)piVar2[7],local_44);
        bVar9 = *puVar4 == local_68;
        FUN_00619730();
        local_4._0_1_ = 2;
        FUN_00619730();
        if (bVar9) {
          *(undefined2 *)((int)this + 0x2e) = 0x429;
          *(undefined2 *)((int)this + 0x30) = 0x478;
          *(undefined4 *)((int)this + 100) = 5;
        }
        else {
          iVar5 = FUN_004edc80((int)piVar2);
          *(ushort *)((int)this + 0x2e) = (iVar5 != 1) + 0x418;
          if (bVar8) {
            *(undefined4 *)((int)this + 100) = 3;
          }
          else {
            *(undefined4 *)((int)this + 100) = 4;
          }
          local_44[1] = 0x33000243;
          FUN_00619730();
          if ((local_70 == 0x33000243) && ((*(byte *)(piVar2 + 0x2c) & 2) != 0)) goto LAB_0049069a;
        }
      }
      else {
        *(undefined2 *)((int)this + 0x2e) = 0x405;
        *(undefined2 *)((int)this + 0x30) = 0x469;
        if (bVar8) {
          *(undefined4 *)((int)this + 100) = 1;
        }
        else {
          *(undefined4 *)((int)this + 100) = 2;
        }
      }
    }
    else {
LAB_0049069a:
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    local_4._0_1_ = 1;
    FUN_00619730();
  }
  iVar5 = local_64;
  if (*(int *)((int)this + 0x40) == 0) goto switchD_0049086b_caseD_244;
  FUN_004f26d0(&local_74,&local_70);
  switch(*(undefined4 *)((int)this + 100)) {
  case 1:
    if (iVar5 == 0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      puVar7 = (undefined4 *)(iVar5 + 0x30);
    }
    if (param_1 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = param_1 + 0xc;
    }
    if (param_2 == (int *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = param_2 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x715c,piVar2,piVar6,puVar7,(undefined4 *)0x0);
    if (iVar5 == 0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      puVar7 = (undefined4 *)(iVar5 + 0x30);
    }
    if (param_1 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = param_1 + 0xc;
    }
    if (param_2 == (int *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = param_2 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x50),0x715d,piVar2,piVar6,puVar7,(undefined4 *)0x0);
    local_44[0xc] = 0x30;
    local_44[0xd] = 0x38;
    if ((local_74 >> 0x18 < 0x30) || (0x37 < local_74 >> 0x18)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    FUN_00619730();
    if (bVar8) {
      switch(local_74 & 0xffffff) {
      case 0x240:
        *(undefined4 *)((int)this + 0x28) = 0x26;
        uVar1 = FUN_004c4990(&DAT_000000f6);
        *(ushort *)((int)this + 0x32) = uVar1;
        break;
      case 0x241:
        *(undefined4 *)((int)this + 0x28) = 0x18;
        uVar1 = FUN_004c4990(&DAT_000000e2);
        *(ushort *)((int)this + 0x32) = uVar1;
        break;
      case 0x242:
        *(undefined4 *)((int)this + 0x28) = 0x25;
        if ((*(byte *)(param_2 + 0x2c) & 8) == 0) {
          uVar1 = FUN_004c4990(&DAT_000000ce);
          *(ushort *)((int)this + 0x32) = uVar1;
        }
        else {
          uVar1 = FUN_004c4990(&DAT_000000d8);
          *(ushort *)((int)this + 0x32) = uVar1;
        }
        break;
      case 0x243:
        *(undefined4 *)((int)this + 0x28) = 0x19;
        uVar1 = FUN_004c4990(&DAT_000000ec);
        *(ushort *)((int)this + 0x32) = uVar1;
        break;
      case 0x280:
        *(undefined4 *)((int)this + 0x28) = 0x27;
        uVar1 = FUN_004c4990(&DAT_0000010a);
        *(ushort *)((int)this + 0x32) = uVar1;
        break;
      case 0x281:
        *(undefined4 *)((int)this + 0x28) = 0x28;
        uVar1 = FUN_004c4990(&DAT_00000100);
        *(ushort *)((int)this + 0x32) = uVar1;
      }
      goto switchD_0049086b_caseD_244;
    }
    break;
  case 2:
    if (iVar5 == 0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      puVar7 = (undefined4 *)(iVar5 + 0x30);
    }
    if (param_1 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = param_1 + 0xc;
    }
    if (param_2 == (int *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = param_2 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x715e,piVar2,piVar6,puVar7,(undefined4 *)0x0);
    if (iVar5 == 0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      puVar7 = (undefined4 *)(iVar5 + 0x30);
    }
    if (param_1 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = param_1 + 0xc;
    }
    if (param_2 == (int *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = param_2 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x50),0x715f,piVar2,piVar6,puVar7,(undefined4 *)0x0);
    local_44[10] = 0x30;
    local_44[0xb] = 0x38;
    if ((local_74 >> 0x18 < 0x30) || (0x37 < local_74 >> 0x18)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    FUN_00619730();
    if (bVar8) {
      switch(local_74 & 0xffffff) {
      case 0x240:
        *(undefined4 *)((int)this + 0x28) = 0x26;
        break;
      case 0x241:
        *(undefined4 *)((int)this + 0x28) = 0x18;
        break;
      case 0x242:
        *(undefined4 *)((int)this + 0x28) = 0x25;
        break;
      case 0x243:
        *(undefined4 *)((int)this + 0x28) = 0x19;
        break;
      case 0x280:
        *(undefined4 *)((int)this + 0x28) = 0x27;
        break;
      case 0x281:
        *(undefined4 *)((int)this + 0x28) = 0x28;
      }
    }
    else {
      *(undefined4 *)((int)this + 0x28) = 0x24;
    }
    goto switchD_0049086b_caseD_244;
  case 3:
    if (iVar5 == 0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      puVar7 = (undefined4 *)(iVar5 + 0x30);
    }
    if (param_1 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = param_1 + 0xc;
    }
    if (param_2 == (int *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = param_2 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x7158,piVar2,piVar6,puVar7,(undefined4 *)0x0);
    if (iVar5 == 0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      puVar7 = (undefined4 *)(iVar5 + 0x30);
    }
    if (param_1 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = param_1 + 0xc;
    }
    if (param_2 == (int *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = param_2 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x50),0x7159,piVar2,piVar6,puVar7,(undefined4 *)0x0);
    local_44[4] = 0x30;
    local_44[5] = 0x38;
    if ((local_74 >> 0x18 < 0x30) || (0x37 < local_74 >> 0x18)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    FUN_00619730();
    if (bVar8) {
      switch(local_74 & 0xffffff) {
      case 0x240:
        goto switchD_0049086b_caseD_240;
      case 0x241:
switchD_0049086b_caseD_241:
        *(undefined4 *)((int)this + 0x28) = 0x20;
        goto switchD_0049086b_caseD_244;
      case 0x242:
switchD_0049086b_caseD_242:
        *(undefined4 *)((int)this + 0x28) = 0x1e;
        goto switchD_0049086b_caseD_244;
      case 0x243:
switchD_00490956_caseD_243:
        *(undefined4 *)((int)this + 0x28) = 0x21;
      default:
        goto switchD_0049086b_caseD_244;
      case 0x280:
switchD_0049086b_caseD_280:
        *(undefined4 *)((int)this + 0x28) = 0x22;
        goto switchD_0049086b_caseD_244;
      case 0x281:
switchD_0049086b_caseD_281:
        *(undefined4 *)((int)this + 0x28) = 0x23;
        goto switchD_0049086b_caseD_244;
      }
    }
    break;
  case 4:
    if (iVar5 == 0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      puVar7 = (undefined4 *)(iVar5 + 0x30);
    }
    if (param_1 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = param_1 + 0xc;
    }
    if (param_2 == (int *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = param_2 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x715a,piVar2,piVar6,puVar7,(undefined4 *)0x0);
    if (iVar5 == 0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      puVar7 = (undefined4 *)(iVar5 + 0x30);
    }
    if (param_1 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = param_1 + 0xc;
    }
    if (param_2 == (int *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = param_2 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x50),0x715b,piVar2,piVar6,puVar7,(undefined4 *)0x0);
    local_44[8] = 0x30;
    local_44[9] = 0x38;
    if ((local_74 >> 0x18 < 0x30) || (0x37 < local_74 >> 0x18)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    FUN_00619730();
    if (bVar8) {
      switch(local_74 & 0xffffff) {
      case 0x240:
        goto switchD_0049086b_caseD_240;
      case 0x241:
        goto switchD_0049086b_caseD_241;
      case 0x242:
        goto switchD_0049086b_caseD_242;
      case 0x243:
        goto switchD_00490956_caseD_243;
      default:
        goto switchD_0049086b_caseD_244;
      case 0x280:
        goto switchD_0049086b_caseD_280;
      case 0x281:
        goto switchD_0049086b_caseD_281;
      }
    }
    break;
  case 5:
    if (param_2 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = param_2 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x7156,piVar6,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    if (param_2 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = param_2 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x50),0x7157,piVar6,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    local_44[6] = 0x30;
    local_44[7] = 0x38;
    if ((local_74 >> 0x18 < 0x30) || (0x37 < local_74 >> 0x18)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    FUN_00619730();
    if (bVar8) {
      switch(local_74 & 0xffffff) {
      case 0x240:
        goto switchD_0049086b_caseD_240;
      case 0x241:
        goto switchD_0049086b_caseD_241;
      case 0x242:
        goto switchD_0049086b_caseD_242;
      case 0x243:
        *(undefined4 *)((int)this + 0x28) = 0x21;
        break;
      default:
        goto switchD_0049086b_caseD_244;
      case 0x280:
        goto switchD_0049086b_caseD_280;
      case 0x281:
        goto switchD_0049086b_caseD_281;
      }
      goto switchD_004906dc_default;
    }
    break;
  default:
switchD_004906dc_default:
    *(undefined4 *)((int)this + 0x40) = 0;
    goto switchD_0049086b_caseD_244;
  }
  *(undefined4 *)((int)this + 0x28) = 0x15;
switchD_0049086b_caseD_244:
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
switchD_0049086b_caseD_240:
  *(undefined4 *)((int)this + 0x28) = 0x1f;
  goto switchD_0049086b_caseD_244;
}

