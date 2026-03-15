
void __thiscall FUN_00491500(void *this,int param_1,int param_2)

{
  bool bVar1;
  ushort uVar2;
  int *piVar3;
  uint uVar4;
  int iVar5;
  uint *puVar6;
  int *piVar7;
  undefined4 *puVar8;
  int *piVar9;
  bool bVar10;
  int *local_a8;
  uint local_a4;
  int *local_a0;
  int *local_9c;
  uint local_98;
  uint local_94;
  uint local_90;
  uint local_8c;
  int local_88;
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
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635d37;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_98);
  piVar9 = (int *)0x0;
  local_4 = 0;
  local_88 = 0;
  if (param_1 != 0x1f1) goto LAB_0049218d;
  FUN_004f26d0(&local_98,(undefined4 *)(param_2 + 0x20));
  piVar3 = FUN_004f3000(*(int *)((int)this + 0x20),&local_98);
  local_a8 = (int *)0x0;
  FUN_004ece30(&local_a4);
  local_4 = CONCAT31(local_4._1_3_,1);
  bVar10 = true;
  *(undefined4 *)((int)this + 0x40) = 0;
  if (piVar3 != (int *)0x0) {
    FUN_004f26d0(&local_a4,piVar3 + 0x1f);
    local_a8 = FUN_004f2d10(*(int *)((int)this + 0x20),&local_a4);
    uVar4 = (uint)piVar3[9] >> 6 & 3;
    if (uVar4 == 1) {
      bVar10 = *(int *)((int)this + 0x20) == 1;
    }
    else if (uVar4 == 2) {
      bVar10 = *(int *)((int)this + 0x20) == 2;
    }
    FUN_004f26d0((uint *)((int)this + 0x5c),piVar3 + 0x1e);
    piVar9 = FUN_004f3220(*(int *)((int)this + 0x20),(uint *)((int)this + 0x5c));
  }
  if (local_a8 == (int *)0x0) {
LAB_004920fd:
    if (!bVar10) {
LAB_00492101:
      if (piVar3 != (int *)0x0) {
        *(undefined4 *)((int)this + 0x40) = 1;
        FUN_0060b9d0((void *)((int)this + 0x44),0x713e,piVar3 + 0xc,(undefined4 *)0x0,
                     (undefined4 *)0x0,(undefined4 *)0x0);
        if (piVar9 == (int *)0x0) {
          piVar9 = (int *)0x0;
        }
        else {
          piVar9 = piVar9 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x713f,piVar3 + 0xc,piVar9,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        *(ushort *)((int)this + 0x2e) = (*(int *)((int)this + 0x20) != 1) + 0x431;
        *(ushort *)((int)this + 0x30) = (*(int *)((int)this + 0x20) != 1) + 0x47c;
      }
    }
  }
  else {
    local_2c = 0x90;
    local_28 = 0x98;
    uVar4 = *(uint *)((int)this + 0x5c) >> 0x18;
    if ((uVar4 < 0x90) || (0x97 < uVar4)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      local_a8 = (int *)0x0;
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    if (local_a8 == (int *)0x0) goto LAB_004920fd;
    if (!bVar10) goto LAB_00492101;
    bVar10 = true;
    *(undefined4 *)((int)this + 0x40) = 1;
    local_a0 = FUN_004f3220(*(int *)((int)this + 0x20),(uint *)((int)this + 0x5c));
    if ((*(byte *)(local_a8 + 0x14) & 8) == 0) {
      local_84 = 0x30;
      local_80 = 0x3c;
      if ((local_a4 >> 0x18 < 0x30) || (0x3b < local_a4 >> 0x18)) {
        bVar10 = false;
      }
      FUN_00619730();
      if ((!bVar10) || ((*(byte *)(local_a8 + 0x2b) & 1) == 0)) {
        FUN_004f26d0((uint *)((int)this + 0x5c),&local_a4);
      }
    }
    piVar9 = (int *)0x0;
    local_9c = (int *)0x0;
    iVar5 = thunk_FUN_005f5060((int)(piVar3 + 0x27));
    bVar10 = true;
    FUN_004ece30(&local_94);
    local_4._0_1_ = 2;
    piVar7 = local_9c;
    while ((iVar5 != 0 && (piVar7 = piVar9, bVar10))) {
      FUN_004f26d0(&local_94,(undefined4 *)(iVar5 + 0x18));
      piVar9 = FUN_004f2d10(*(int *)((int)this + 0x20),&local_94);
      if (piVar9 != (int *)0x0) {
        if ((*(byte *)(piVar9 + 0x14) & 8) == 0) {
          local_6c = 0x30;
          local_68 = 0x3c;
          if ((local_94 >> 0x18 < 0x30) || (0x3b < local_94 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if ((bVar1) && ((*(byte *)(piVar9 + 0x2b) & 1) != 0)) goto LAB_004917a6;
        }
        else {
LAB_004917a6:
          piVar9 = (int *)0x0;
        }
        while( true ) {
          while( true ) {
            if ((piVar9 == (int *)0x0) || (!bVar10)) goto LAB_0049188c;
            puVar6 = FUN_004025b0(piVar9,&local_90);
            local_3c = 0x90;
            local_38 = 0x98;
            local_4._0_1_ = 3;
            if ((*puVar6 >> 0x18 < 0x90) || (0x97 < *puVar6 >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            local_4._0_1_ = 2;
            FUN_00619730();
            if (!bVar1) break;
            bVar10 = false;
          }
          puVar6 = FUN_004025b0(piVar9,&local_8c);
          local_5c = 8;
          local_58 = 0x10;
          local_4._0_1_ = 4;
          if ((*puVar6 >> 0x18 < 8) || (0xf < *puVar6 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          local_4._0_1_ = 2;
          FUN_00619730();
          if (bVar1) break;
          piVar9 = (int *)piVar9[7];
        }
        bVar10 = false;
      }
LAB_0049188c:
      iVar5 = *(int *)(iVar5 + 0x10);
      piVar7 = piVar9;
    }
    local_9c = piVar7;
    local_1c = 0x30;
    local_18 = 0x38;
    if ((local_a4 >> 0x18 < 0x30) || (0x37 < local_a4 >> 0x18)) {
      bVar10 = false;
    }
    else {
      bVar10 = true;
    }
    FUN_00619730();
    if (bVar10) {
      switch(local_a4 & 0xffffff) {
      case 0x240:
        *(undefined4 *)((int)this + 0x28) = 0x17;
        uVar2 = FUN_004c4990(&DAT_000000f3);
        *(ushort *)((int)this + 0x32) = uVar2;
        break;
      case 0x241:
        *(undefined4 *)((int)this + 0x28) = 0x18;
        uVar2 = FUN_004c4990(&DAT_000000df);
        *(ushort *)((int)this + 0x32) = uVar2;
        break;
      case 0x242:
        *(undefined4 *)((int)this + 0x28) = 0x16;
        if ((*(byte *)(local_a8 + 0x2c) & 8) == 0) {
          uVar2 = FUN_004c4990(&DAT_000000cb);
          *(ushort *)((int)this + 0x32) = uVar2;
        }
        else {
          uVar2 = FUN_004c4990(&DAT_000000d5);
          *(ushort *)((int)this + 0x32) = uVar2;
        }
        break;
      case 0x243:
        *(undefined4 *)((int)this + 0x28) = 0x19;
        uVar2 = FUN_004c4990(&DAT_000000e9);
        *(ushort *)((int)this + 0x32) = uVar2;
        break;
      default:
        break;
      case 0x280:
        *(undefined4 *)((int)this + 0x28) = 0x1a;
        uVar2 = FUN_004c4990(&DAT_00000107);
        *(ushort *)((int)this + 0x32) = uVar2;
        break;
      case 0x281:
        *(undefined4 *)((int)this + 0x28) = 0x1b;
        uVar2 = FUN_004c4990(&DAT_000000fd);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
    }
    else {
      local_4c = 0x38;
      local_48 = 0x3c;
      if ((local_a4 >> 0x18 < 0x38) || (0x3b < local_a4 >> 0x18)) {
        bVar10 = false;
      }
      else {
        bVar10 = true;
      }
      FUN_00619730();
      if (bVar10) {
        *(undefined4 *)((int)this + 0x28) = 0x15;
      }
    }
    local_7c = 0x30;
    *(ushort *)((int)this + 0x2e) = (*(int *)((int)this + 0x20) != 1) + 0x412;
    *(ushort *)((int)this + 0x30) = (*(int *)((int)this + 0x20) != 1) + 0x47c;
    local_78 = 0x3c;
    if ((local_a4 >> 0x18 < 0x30) || (0x3b < local_a4 >> 0x18)) {
      bVar10 = false;
    }
    else {
      bVar10 = true;
    }
    FUN_00619730();
    if (bVar10) {
      uVar4 = FUN_004c5000(local_a8);
      *(short *)((int)this + 0x2c) = (short)uVar4;
      iVar5 = local_88;
    }
    else {
      uVar4 = FUN_004c5000(local_a8);
      *(short *)((int)this + 0x2e) = (short)uVar4;
      local_74 = 0x54;
      local_70 = 0x55;
      if ((local_98 >> 0x18 < 0x54) || (0x54 < local_98 >> 0x18)) {
        iVar5 = 0;
      }
      else {
        iVar5 = 1;
      }
      FUN_00619730();
    }
    switch(*(undefined4 *)(param_2 + 0x28)) {
    case 0:
    case 1:
    case 2:
      *(undefined4 *)((int)this + 0x40) = 0;
      break;
    case 3:
    case 4:
    case 5:
      *(undefined2 *)((int)this + 0x30) = 0x480;
      if (iVar5 == 0) {
        *(ushort *)((int)this + 0x2e) = (*(int *)((int)this + 0x20) != 1) + 0x416;
      }
      local_64 = 0x30;
      local_60 = 0x38;
      if ((local_a4 >> 0x18 < 0x30) || (0x37 < local_a4 >> 0x18)) {
        bVar10 = false;
      }
      else {
        bVar10 = true;
      }
      FUN_00619730();
      if (bVar10) {
        if (piVar3 == (int *)0x0) {
          piVar7 = (int *)0x0;
        }
        else {
          piVar7 = piVar3 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x44),29000,local_a8 + 0xc,piVar7,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        if (local_a0 == (int *)0x0) {
          piVar7 = (int *)0x0;
        }
        else {
          piVar7 = local_a0 + 0xc;
        }
        if (piVar3 == (int *)0x0) {
          piVar3 = (int *)0x0;
        }
        else {
          piVar3 = piVar3 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x7149,local_a8 + 0xc,piVar3,piVar7,
                     (undefined4 *)0x0);
      }
      else {
        if (piVar3 == (int *)0x0) {
          piVar7 = (int *)0x0;
        }
        else {
          piVar7 = piVar3 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x44),0x714a,local_a8 + 0xc,piVar7,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        if (local_a0 == (int *)0x0) {
          piVar7 = (int *)0x0;
        }
        else {
          piVar7 = local_a0 + 0xc;
        }
        if (piVar3 == (int *)0x0) {
          piVar3 = (int *)0x0;
        }
        else {
          piVar3 = piVar3 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x714b,local_a8 + 0xc,piVar3,piVar7,
                     (undefined4 *)0x0);
      }
      break;
    case 6:
    case 7:
    case 9:
    case 10:
      local_34 = 0x53;
      local_30 = 0x54;
      if ((local_98 >> 0x18 < 0x53) || (0x53 < local_98 >> 0x18)) {
        bVar10 = false;
      }
      else {
        bVar10 = true;
      }
      FUN_00619730();
      if (bVar10) {
        *(undefined4 *)((int)this + 0x40) = 0;
        piVar9 = local_9c;
      }
      else {
        local_24 = 0x30;
        local_20 = 0x38;
        if ((local_a4 >> 0x18 < 0x30) || (0x37 < local_a4 >> 0x18)) {
          bVar10 = false;
        }
        else {
          bVar10 = true;
        }
        FUN_00619730();
        if (bVar10) {
          if (piVar3 == (int *)0x0) {
            piVar9 = (int *)0x0;
          }
          else {
            piVar9 = piVar3 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x44),0x7138,local_a8 + 0xc,piVar9,(undefined4 *)0x0,
                       (undefined4 *)0x0);
          if (local_a0 == (int *)0x0) {
            piVar9 = (int *)0x0;
          }
          else {
            piVar9 = local_a0 + 0xc;
          }
          if (piVar3 == (int *)0x0) {
            piVar3 = (int *)0x0;
            uVar4 = 0x7139;
          }
          else {
            piVar3 = piVar3 + 0xc;
            uVar4 = 0x7139;
          }
        }
        else {
          if (piVar3 == (int *)0x0) {
            piVar9 = (int *)0x0;
          }
          else {
            piVar9 = piVar3 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x44),0x713a,local_a8 + 0xc,piVar9,(undefined4 *)0x0,
                       (undefined4 *)0x0);
          if (local_a0 == (int *)0x0) {
            piVar9 = (int *)0x0;
          }
          else {
            piVar9 = local_a0 + 0xc;
          }
          if (piVar3 == (int *)0x0) {
            piVar3 = (int *)0x0;
          }
          else {
            piVar3 = piVar3 + 0xc;
          }
          uVar4 = 0x713b;
        }
LAB_00491eb1:
        FUN_0060b9d0((void *)((int)this + 0x50),uVar4,local_a8 + 0xc,piVar3,piVar9,(undefined4 *)0x0
                    );
        piVar9 = local_9c;
      }
      break;
    case 8:
      if (((byte)local_a0[9] & 0xc0) == 0x40) {
        uVar4 = 1;
        iVar5 = *(int *)((int)this + 0x20);
      }
      else {
        uVar4 = 2;
        iVar5 = *(int *)((int)this + 0x20);
      }
      iVar5 = FUN_004f3dd0(iVar5,uVar4);
      local_14 = 0x30;
      local_10 = 0x38;
      if ((local_a4 >> 0x18 < 0x30) || (0x37 < local_a4 >> 0x18)) {
        bVar10 = false;
      }
      else {
        bVar10 = true;
      }
      FUN_00619730();
      if (bVar10) {
        if (piVar3 == (int *)0x0) {
          piVar7 = (int *)0x0;
        }
        else {
          piVar7 = piVar3 + 0xc;
        }
        local_a8 = local_a8 + 0xc;
        FUN_0060b9d0((void *)((int)this + 0x44),0x7140,local_a8,piVar7,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        if (((byte)local_a0[9] & 0xc0) == 0xc0) {
          if (local_a0 == (int *)0x0) {
            piVar7 = (int *)0x0;
          }
          else {
            piVar7 = local_a0 + 0xc;
          }
          if (piVar3 == (int *)0x0) {
            piVar3 = (int *)0x0;
          }
          else {
            piVar3 = piVar3 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x50),0x7144,local_a8,piVar3,piVar7,(undefined4 *)0x0);
        }
        else {
          if (iVar5 == 0) {
            puVar8 = (undefined4 *)0x0;
          }
          else {
            puVar8 = (undefined4 *)(iVar5 + 0x30);
          }
          if (local_a0 == (int *)0x0) {
            piVar7 = (int *)0x0;
          }
          else {
            piVar7 = local_a0 + 0xc;
          }
          if (piVar3 == (int *)0x0) {
            piVar3 = (int *)0x0;
          }
          else {
            piVar3 = piVar3 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x50),0x7141,local_a8,piVar3,piVar7,puVar8);
        }
      }
      else {
        if (piVar3 == (int *)0x0) {
          piVar7 = (int *)0x0;
        }
        else {
          piVar7 = piVar3 + 0xc;
        }
        local_a8 = local_a8 + 0xc;
        FUN_0060b9d0((void *)((int)this + 0x44),0x7142,local_a8,piVar7,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        if (((byte)local_a0[9] & 0xc0) == 0xc0) {
          if (local_a0 == (int *)0x0) {
            piVar7 = (int *)0x0;
          }
          else {
            piVar7 = local_a0 + 0xc;
          }
          if (piVar3 == (int *)0x0) {
            piVar3 = (int *)0x0;
          }
          else {
            piVar3 = piVar3 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x50),0x7145,local_a8,piVar3,piVar7,(undefined4 *)0x0);
        }
        else {
          if (iVar5 == 0) {
            puVar8 = (undefined4 *)0x0;
          }
          else {
            puVar8 = (undefined4 *)(iVar5 + 0x30);
          }
          if (local_a0 == (int *)0x0) {
            piVar7 = (int *)0x0;
          }
          else {
            piVar7 = local_a0 + 0xc;
          }
          if (piVar3 == (int *)0x0) {
            piVar3 = (int *)0x0;
          }
          else {
            piVar3 = piVar3 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x50),0x7143,local_a8,piVar3,piVar7,puVar8);
        }
      }
      break;
    default:
      *(undefined4 *)((int)this + 0x40) = 0;
      break;
    case 0xc:
      local_54 = 0x53;
      local_50 = 0x54;
      if ((local_98 >> 0x18 < 0x53) || (0x53 < local_98 >> 0x18)) {
        bVar10 = false;
      }
      else {
        bVar10 = true;
      }
      FUN_00619730();
      if (bVar10) {
        local_44 = 0x30;
        local_40 = 0x38;
        if ((local_a4 >> 0x18 < 0x30) || (0x37 < local_a4 >> 0x18)) {
          bVar10 = false;
        }
        else {
          bVar10 = true;
        }
        FUN_00619730();
        if (bVar10) {
          if (piVar3 == (int *)0x0) {
            piVar9 = (int *)0x0;
          }
          else {
            piVar9 = piVar3 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x44),0x7138,local_a8 + 0xc,piVar9,(undefined4 *)0x0,
                       (undefined4 *)0x0);
          if (local_a0 == (int *)0x0) {
            piVar9 = (int *)0x0;
          }
          else {
            piVar9 = local_a0 + 0xc;
          }
          if (piVar3 == (int *)0x0) {
            piVar3 = (int *)0x0;
            uVar4 = 0x713c;
          }
          else {
            piVar3 = piVar3 + 0xc;
            uVar4 = 0x713c;
          }
        }
        else {
          if (piVar3 == (int *)0x0) {
            piVar9 = (int *)0x0;
          }
          else {
            piVar9 = piVar3 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x44),0x713a,local_a8 + 0xc,piVar9,(undefined4 *)0x0,
                       (undefined4 *)0x0);
          if (local_a0 == (int *)0x0) {
            piVar9 = (int *)0x0;
          }
          else {
            piVar9 = local_a0 + 0xc;
          }
          if (piVar3 == (int *)0x0) {
            piVar3 = (int *)0x0;
            uVar4 = 0x713d;
          }
          else {
            piVar3 = piVar3 + 0xc;
            uVar4 = 0x713d;
          }
        }
        goto LAB_00491eb1;
      }
      *(undefined4 *)((int)this + 0x40) = 0;
      piVar9 = local_9c;
    }
    if (piVar9 == (int *)0x0) {
      piVar9 = (int *)0x0;
      uVar4 = 0x70e7;
    }
    else {
      piVar9 = piVar9 + 0xc;
      uVar4 = 0x70e6;
    }
    FUN_0060b9d0((void *)((int)this + 0x50),uVar4,piVar9,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
LAB_0049218d:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

