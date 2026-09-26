
void __thiscall FUN_004955b0(void *this,void *param_1,int param_2)

{
  void *pvVar1;
  ushort uVar2;
  uint *puVar3;
  int *piVar4;
  int iVar5;
  undefined4 *puVar6;
  void *this_00;
  bool bVar7;
  uint uVar8;
  int *piVar9;
  uint local_54;
  int *local_50;
  int local_4c;
  int local_48;
  uint local_44;
  uint local_40;
  uint local_3c;
  undefined1 local_38 [4];
  uint local_34;
  uint local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636110;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_54);
  local_4 = 0;
  if (param_1 == (void *)0x200) {
    FUN_004f26d0(&local_54,(undefined4 *)(param_2 + 0x20));
    FUN_004f26d0(&local_44,(undefined4 *)(param_2 + 0x2c));
    local_4._0_1_ = 1;
    local_50 = FUN_004f2d10(*(int *)((int)this + 0x20),&local_54);
    param_1 = (void *)0x0;
    local_48 = 1;
    if (*(int *)((int)this + 100) == 0) {
      *(undefined4 *)((int)this + 100) = 1;
      *(ushort *)((int)this + 0x2e) = (*(int *)((int)this + 0x20) != 1) + 0x412;
      *(ushort *)((int)this + 0x30) = (*(int *)((int)this + 0x20) != 1) + 0x47c;
    }
    if (((local_50 == (int *)0x0) || (*(int *)(param_2 + 0x28) != 0)) ||
       ((*(byte *)(local_50 + 0x14) & 0x20) != 0)) {
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    else {
      this_00 = (void *)local_50[7];
      do {
        if (this_00 == (void *)0x0) break;
        puVar3 = FUN_004025b0(this_00,&local_40);
        local_2c = 0x90;
        local_28 = 0x98;
        local_4._0_1_ = 2;
        if ((*puVar3 >> 0x18 < 0x90) || (0x97 < *puVar3 >> 0x18)) {
          bVar7 = false;
        }
        else {
          bVar7 = true;
        }
        FUN_00619730();
        local_4._0_1_ = 1;
        FUN_00619730();
        pvVar1 = this_00;
        if (!bVar7) {
          puVar3 = FUN_004025b0(this_00,&local_3c);
          local_24 = 0x98;
          local_20 = 0xa0;
          local_4._0_1_ = 3;
          if ((*puVar3 >> 0x18 < 0x98) || (0x9f < *puVar3 >> 0x18)) {
            bVar7 = false;
          }
          else {
            bVar7 = true;
          }
          FUN_00619730();
          local_4._0_1_ = 1;
          FUN_00619730();
          pvVar1 = param_1;
          if (bVar7) {
            this_00 = (void *)0x0;
            *(undefined4 *)((int)this + 0x40) = 0;
          }
          else {
            this_00 = *(void **)((int)this_00 + 0x1c);
          }
        }
        param_1 = pvVar1;
      } while (param_1 == (void *)0x0);
      if (param_1 == (void *)0x0) {
        *(undefined4 *)((int)this + 0x40) = 0;
      }
      else {
        *(undefined4 *)((int)this + 0x40) = 1;
      }
      local_1c = 0x40;
      local_18 = 0x50;
      if ((local_44 >> 0x18 < 0x40) || (0x4f < local_44 >> 0x18)) {
        bVar7 = false;
      }
      else {
        bVar7 = true;
      }
      FUN_00619730();
      if (!bVar7) {
        *(undefined4 *)((int)this + 0x40) = 0;
      }
      local_4c = 0x3c000009;
      local_4._0_1_ = 4;
      piVar4 = FUN_00402d80(local_50,local_38);
      bVar7 = *piVar4 == local_4c;
      FUN_00619730();
      local_4._0_1_ = 1;
      FUN_00619730();
      if (bVar7) {
        *(undefined4 *)((int)this + 0x40) = 0;
      }
    }
    if (*(int *)((int)this + 0x40) != 0) {
      if (*(int *)(param_2 + 0x30) != 0) {
        puVar3 = (uint *)((int)this + 0x74);
        FUN_004f26d0(puVar3,&local_54);
        FUN_004f26d0((void *)((int)this + 0x5c),&local_54);
        local_14 = 0x30;
        local_10 = 0x38;
        if ((*puVar3 >> 0x18 < 0x30) || (0x37 < *puVar3 >> 0x18)) {
          bVar7 = false;
        }
        else {
          bVar7 = true;
        }
        FUN_00619730();
        piVar4 = local_50;
        if (bVar7) {
          if (param_1 == (void *)0x0) {
            puVar6 = (undefined4 *)0x0;
          }
          else {
            puVar6 = (undefined4 *)((int)param_1 + 0x30);
          }
          if (local_50 == (int *)0x0) {
            piVar9 = (int *)0x0;
          }
          else {
            piVar9 = local_50 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x44),0x70c1,piVar9,puVar6,(undefined4 *)0x0,
                       (undefined4 *)0x0);
          if (*(int *)((int)this + 0x78) < 2) {
            if (param_1 == (void *)0x0) {
              puVar6 = (undefined4 *)0x0;
            }
            else {
              puVar6 = (undefined4 *)((int)param_1 + 0x30);
            }
            if (piVar4 == (int *)0x0) {
              piVar9 = (int *)0x0;
            }
            else {
              piVar9 = piVar4 + 0xc;
            }
            uVar8 = 0x70c5;
          }
          else {
            if (param_1 == (void *)0x0) {
              puVar6 = (undefined4 *)0x0;
            }
            else {
              puVar6 = (undefined4 *)((int)param_1 + 0x30);
            }
            if (piVar4 == (int *)0x0) {
              piVar9 = (int *)0x0;
              uVar8 = 0x70c4;
            }
            else {
              piVar9 = piVar4 + 0xc;
              uVar8 = 0x70c4;
            }
          }
          FUN_0060b9d0((void *)((int)this + 0x50),uVar8,piVar9,puVar6,(undefined4 *)0x0,
                       (undefined4 *)0x0);
          local_48 = 0;
          switch(*puVar3 & 0xffffff) {
          case 0x240:
            *(undefined4 *)((int)this + 0x28) = 0x17;
            uVar2 = FUN_004c4990(&DAT_000000f0);
            *(ushort *)((int)this + 0x32) = uVar2;
            break;
          case 0x241:
            *(undefined4 *)((int)this + 0x28) = 0x18;
            uVar2 = FUN_004c4990(&DAT_000000dc);
            *(ushort *)((int)this + 0x32) = uVar2;
            break;
          case 0x242:
            *(undefined4 *)((int)this + 0x28) = 0x16;
            if ((*(byte *)(piVar4 + 0x2c) & 8) == 0) {
              uVar2 = FUN_004c4990(&DAT_000000c8);
              *(ushort *)((int)this + 0x32) = uVar2;
            }
            else {
              uVar2 = FUN_004c4990(&DAT_000000d2);
              *(ushort *)((int)this + 0x32) = uVar2;
            }
            break;
          case 0x243:
            *(undefined4 *)((int)this + 0x28) = 0x19;
            uVar2 = FUN_004c4990(&DAT_000000e6);
            *(ushort *)((int)this + 0x32) = uVar2;
            break;
          case 0x280:
            *(undefined4 *)((int)this + 0x28) = 0x1a;
            uVar2 = FUN_004c4990(&DAT_00000104);
            *(ushort *)((int)this + 0x32) = uVar2;
            break;
          case 0x281:
            *(undefined4 *)((int)this + 0x28) = 0x1b;
            uVar2 = FUN_004c4990(&DAT_000000fa);
            *(ushort *)((int)this + 0x32) = uVar2;
          }
        }
        else {
          if (param_1 == (void *)0x0) {
            puVar6 = (undefined4 *)0x0;
          }
          else {
            puVar6 = (undefined4 *)((int)param_1 + 0x30);
          }
          if (local_50 == (int *)0x0) {
            piVar9 = (int *)0x0;
          }
          else {
            piVar9 = local_50 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x44),0x70c0,piVar9,puVar6,(undefined4 *)0x0,
                       (undefined4 *)0x0);
          if (param_1 == (void *)0x0) {
            puVar6 = (undefined4 *)0x0;
          }
          else {
            puVar6 = (undefined4 *)((int)param_1 + 0x30);
          }
          if (piVar4 == (int *)0x0) {
            piVar9 = (int *)0x0;
          }
          else {
            piVar9 = piVar4 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x50),0x70c3,piVar9,puVar6,(undefined4 *)0x0,
                       (undefined4 *)0x0);
          *(undefined4 *)((int)this + 0x28) = 0x15;
        }
        local_34 = 0x30;
        local_30 = 0x3c;
        local_4._0_1_ = 5;
        uVar8 = (**(code **)(*piVar4 + 4))();
        if ((uVar8 < local_34) || (local_30 <= uVar8)) {
          bVar7 = false;
        }
        else {
          bVar7 = true;
        }
        local_4._0_1_ = 1;
        FUN_00619730();
        if (bVar7) {
          uVar8 = FUN_004c5000(piVar4);
          *(short *)((int)this + 0x2c) = (short)uVar8;
        }
      }
      if ((*(int *)((int)this + 0x40) != 0) && (local_48 != 0)) {
        if (param_1 == (void *)0x0) {
          puVar6 = (undefined4 *)0x0;
        }
        else {
          puVar6 = (undefined4 *)((int)param_1 + 0x30);
        }
        if (local_50 == (int *)0x0) {
          piVar4 = (int *)0x0;
        }
        else {
          piVar4 = local_50 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x68),0x70c2,piVar4,puVar6,(undefined4 *)0x0,
                     (undefined4 *)0x0);
      }
    }
    iVar5 = FUN_004ece60((uint *)((int)this + 0x5c));
    if (iVar5 == 0) {
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

