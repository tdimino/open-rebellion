
int __thiscall FUN_00488de0(void *this,int param_1)

{
  int *this_00;
  void **ppvVar1;
  uint uVar2;
  int *piVar3;
  uint *puVar4;
  int iVar5;
  int iVar6;
  bool bVar7;
  int local_84;
  uint uStack_80;
  uint uStack_7c;
  uint uStack_78;
  uint uStack_74;
  uint uStack_70;
  undefined1 auStack_6c [4];
  undefined4 uStack_68;
  undefined1 auStack_64 [4];
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *local_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00634c80;
  local_c = ExceptionList;
  iVar6 = 0;
  local_84 = 0;
  if (param_1 != 0) {
    ppvVar1 = &local_c;
    for (this_00 = *(int **)(param_1 + 8); ExceptionList = ppvVar1, this_00 != (int *)0x0;
        this_00 = (int *)this_00[2]) {
      if (iVar6 != 0) {
        ExceptionList = local_c;
        return iVar6;
      }
      uVar2 = (**(code **)(*this_00 + 0xc))();
      if (uVar2 < 0x101) {
        if (uVar2 == 0x100) {
          FUN_004f26d0(&uStack_7c,this_00 + 8);
          uStack_5c = 0x90;
          uStack_58 = 0x98;
          if ((uStack_7c >> 0x18 < 0x90) || (0x97 < uStack_7c >> 0x18)) {
            bVar7 = false;
          }
          else {
            bVar7 = true;
          }
          iStack_4 = iVar6;
          FUN_00619730();
          if (bVar7) {
            iVar6 = 4;
          }
          iStack_4 = 0xffffffff;
          FUN_00619730();
        }
      }
      else if (uVar2 < 0x128) {
        if (uVar2 < 0x125) {
          if (uVar2 == 0x105) {
            FUN_004ece30(&uStack_78);
            iStack_4 = 1;
            if ((this_00[10] == 0) &&
               (piVar3 = FUN_004f2d10(*(int *)this,(uint *)(this_00 + 8)), piVar3 != (int *)0x0)) {
              if ((piVar3[0x10] & 0xff00U) == 0x100) {
                puVar4 = FUN_004025b0(piVar3,&uStack_70);
                bVar7 = *puVar4 != uStack_78;
                FUN_00619730();
                if (bVar7) {
                  iVar6 = 0;
                  goto LAB_00488fad;
                }
              }
              iVar6 = 0xd;
            }
LAB_00488fad:
            iStack_4 = 0xffffffff;
            FUN_00619730();
          }
          else if (uVar2 == 0x106) {
            uStack_54 = 0x30;
            uStack_50 = 0x3c;
            if (((uint)this_00[8] >> 0x18 < 0x30) || (0x3b < (uint)this_00[8] >> 0x18)) {
              bVar7 = false;
            }
            else {
              bVar7 = true;
            }
            FUN_00619730();
            if (bVar7) {
              iVar6 = 0x1a;
            }
            else if (this_00[10] == 0) {
              iVar6 = 6;
            }
          }
          else if (uVar2 == 0x107) {
            iVar6 = 0xe;
          }
        }
        else {
          iVar6 = 5;
        }
      }
      else if (uVar2 < 0x146) {
        if (uVar2 == 0x145) {
          iVar6 = 9;
        }
        else {
          switch(uVar2) {
          case 0x128:
            if (((this_00[10] & 1U) == 0) || ((*(byte *)(this_00 + 9) & 1) != 0)) {
              if (((this_00[10] & 0x100U) != 0) && ((this_00[9] & 0x100U) == 0)) {
                iVar6 = 0x1e;
              }
            }
            else {
              iVar6 = 0x1f;
            }
            break;
          case 0x129:
          case 0x12a:
          case 299:
            iVar6 = 0x2c;
            break;
          case 300:
            iVar6 = 0x2d;
          }
        }
      }
      else if (uVar2 < 0x14c) {
        if (uVar2 == 0x14b) {
          iVar6 = 3;
        }
        else if (uVar2 == 0x148) {
          iVar6 = 0xb;
        }
      }
      else if (uVar2 < 0x161) {
        if (uVar2 == 0x160) {
          if (this_00[10] == 0) {
            iVar6 = 0x21;
          }
        }
        else {
          switch(uVar2) {
          case 0x14d:
            iVar6 = 1;
            break;
          case 0x14e:
            iVar6 = 7;
            break;
          case 0x14f:
            if (this_00[10] == 0) {
              puVar4 = FUN_0042ec10(this_00,auStack_6c);
              iStack_4 = 4;
              piVar3 = FUN_004f3220(*(int *)this,puVar4);
              iStack_4 = 0xffffffff;
              FUN_00619730();
              iVar6 = 0xf;
              if ((piVar3 != (int *)0x0) && ((*(byte *)(piVar3 + 0x14) & 8) != 0)) {
                iVar6 = 0;
              }
            }
            break;
          case 0x150:
            iVar5 = this_00[8];
            iVar6 = 0xf;
            uStack_68 = 0x90000109;
            FUN_00619730();
            if (((iVar5 == -0x6ffffef7) && (this_00[10] != 0)) && (this_00[9] == 2)) {
              uStack_74 = 0x90000109;
              iStack_4 = 5;
              piVar3 = FUN_004f3220(*(int *)this,&uStack_74);
              iStack_4 = 0xffffffff;
              FUN_00619730();
              if (((byte)piVar3[9] & 0xc0) == 0x40) {
                iVar6 = 0;
              }
            }
            break;
          case 0x153:
            iVar6 = 0x18;
            break;
          case 0x154:
          case 0x155:
            iVar6 = 0x20;
          }
        }
      }
      else if (uVar2 < 0x1f1) {
        if (uVar2 == 0x1f0) {
          puVar4 = (uint *)(this_00 + 8);
          piVar3 = FUN_004f3000(*(int *)this,puVar4);
          if ((piVar3 != (int *)0x0) && (iVar5 = FUN_00520b70((int)piVar3), iVar5 == 0)) {
            uStack_4c = 0x40;
            uStack_48 = 0x50;
            if ((*puVar4 >> 0x18 < 0x40) || (0x4f < *puVar4 >> 0x18)) {
              bVar7 = false;
            }
            else {
              bVar7 = true;
            }
            FUN_00619730();
            if ((!bVar7) && (this_00[10] != 0)) {
              iVar6 = 0x16;
            }
            uStack_44 = 0x6a;
            uStack_40 = 0x6b;
            if ((*puVar4 >> 0x18 < 0x6a) || (0x6a < *puVar4 >> 0x18)) {
              bVar7 = false;
            }
            else {
              bVar7 = true;
            }
            FUN_00619730();
            if ((bVar7) && (this_00[10] == 3)) {
              iVar6 = 0;
            }
            if (*(int *)this == 1) {
              if (((byte)piVar3[9] & 0xc0) != 0x40) {
                iVar6 = 0;
              }
            }
            else if ((*(int *)this == 2) && (((byte)piVar3[9] & 0xc0) != 0x80)) {
              iVar6 = 0;
            }
          }
        }
        else if (uVar2 == 0x1e0) {
          iVar6 = 0x19;
        }
        else if (uVar2 == 0x1e1) {
          iVar6 = 0x1d;
        }
      }
      else if (uVar2 < 0x201) {
        if (uVar2 == 0x200) {
          uStack_34 = 0x40;
          uStack_30 = 0x50;
          if (((uint)this_00[0xb] >> 0x18 < 0x40) || (0x4f < (uint)this_00[0xb] >> 0x18)) {
            bVar7 = false;
          }
          else {
            bVar7 = true;
          }
          FUN_00619730();
          if (bVar7) {
            if ((this_00[10] == 0) && (local_84 == 0)) {
              FUN_0042ec10(this_00,&uStack_80);
              iStack_4 = 2;
              uStack_2c = 0x3c;
              uStack_28 = 0x40;
              if ((uStack_80 >> 0x18 < 0x3c) || (0x3f < uStack_80 >> 0x18)) {
                bVar7 = false;
              }
              else {
                bVar7 = true;
              }
              FUN_00619730();
              if (bVar7) {
                piVar3 = FUN_004f3af0(*(int *)this,&uStack_80);
                if ((piVar3 != (int *)0x0) && (*(byte *)((int)piVar3 + 0x41) < 2)) {
                  FUN_004ece30(&uStack_60);
                  iStack_4._0_1_ = 3;
                  FUN_0048a640(piVar3,auStack_64);
                  FUN_00619730();
                  iStack_4 = CONCAT31(iStack_4._1_3_,2);
                  FUN_00619730();
                }
              }
              else {
                local_84 = 0x14;
              }
              iStack_4 = 0xffffffff;
              FUN_00619730();
            }
          }
          else {
            uStack_24 = 100;
            uStack_20 = 0x65;
            if (((uint)this_00[0xb] >> 0x18 < 100) || (100 < (uint)this_00[0xb] >> 0x18)) {
              bVar7 = false;
            }
            else {
              bVar7 = true;
            }
            FUN_00619730();
            if ((bVar7) && (this_00[10] != 0)) {
              iVar6 = 0x28;
            }
          }
        }
        else if ((uVar2 == 0x1f1) && (this_00[10] != 0)) {
          uStack_3c = 0x40;
          uStack_38 = 0x50;
          if (((uint)this_00[8] >> 0x18 < 0x40) || (0x4f < (uint)this_00[8] >> 0x18)) {
            bVar7 = false;
          }
          else {
            bVar7 = true;
          }
          FUN_00619730();
          if ((((!bVar7) &&
               (piVar3 = FUN_004f3000(*(int *)this,(uint *)(this_00 + 8)), piVar3 != (int *)0x0)) &&
              (iVar5 = FUN_00520b70((int)piVar3), iVar5 == 0)) && (piVar3[0x18] == 0)) {
            iVar6 = 0x17;
          }
        }
      }
      else if (uVar2 < 0x221) {
        if (uVar2 == 0x220) {
switchD_004895c4_caseD_360:
          iVar6 = 0x24;
        }
        else if (uVar2 == 0x210) {
LAB_00489532:
          iVar6 = 0x25;
        }
        else if (uVar2 == 0x212) {
          iVar6 = 0x1b;
        }
      }
      else if (uVar2 < 0x231) {
        if (uVar2 == 0x230) {
          iVar6 = 0x26;
        }
        else if (uVar2 == 0x221) goto LAB_00489532;
      }
      else if (uVar2 < 0x303) {
        if (uVar2 == 0x302) {
          uStack_1c = 0x90;
          uStack_18 = 0x98;
          if (((uint)this_00[8] >> 0x18 < 0x90) || (0x97 < (uint)this_00[8] >> 0x18)) {
            bVar7 = false;
          }
          else {
            bVar7 = true;
          }
          FUN_00619730();
          if (bVar7) {
            iVar6 = 0x15;
          }
        }
        else if (uVar2 == 0x231) {
          iVar6 = 0x29;
        }
      }
      else {
        switch(uVar2) {
        case 0x303:
          iVar6 = 0x10;
          break;
        case 0x304:
          iVar6 = 0xc;
          break;
        case 0x305:
          uStack_14 = 0x18;
          uStack_10 = 0x1c;
          if (((uint)this_00[8] >> 0x18 < 0x18) || (0x1b < (uint)this_00[8] >> 0x18)) {
            bVar7 = false;
          }
          else {
            bVar7 = true;
          }
          FUN_00619730();
          if (bVar7) {
            iVar6 = 0x23;
          }
          else {
            local_84 = 0x22;
          }
          break;
        case 0x306:
          iVar6 = 0x2b;
          break;
        case 0x340:
          iVar6 = 0x13;
          break;
        case 0x360:
          goto switchD_004895c4_caseD_360;
        case 0x361:
          iVar6 = 0x27;
          break;
        case 0x362:
          iVar6 = 0x1c;
        }
      }
      ppvVar1 = ExceptionList;
    }
  }
  if (iVar6 == 0) {
    iVar6 = local_84;
  }
  ExceptionList = local_c;
  return iVar6;
}

