
uint __thiscall FUN_00501c50(int *param_1,void *param_2)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int iVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  void *pvVar8;
  int aiStack_8 [2];
  
  uVar7 = 1;
  if (0 < param_1[0x18]) {
    pvVar8 = param_2;
    iVar2 = (**(code **)(*param_1 + 0x248))();
    bVar1 = FUN_00501b70(param_1,iVar2 / 10,pvVar8);
    uVar7 = CONCAT31(extraout_var,bVar1);
  }
  aiStack_8[0] = 0;
  uVar6 = uVar7;
  if (uVar7 != 0) {
    iVar2 = FUN_005006a0((int)param_1);
    if (iVar2 != 0) {
      iVar3 = (**(code **)(*param_1 + 0x28c))();
      iVar4 = (**(code **)(*param_1 + 0x260))();
      uVar7 = FUN_00558680(iVar4 - iVar3,aiStack_8);
    }
    uVar6 = uVar7;
    if ((uVar7 != 0) && (aiStack_8[0] != 0)) {
      iVar2 = FUN_0053e290(iVar2 + -1);
      uVar5 = param_1[0x19] & 0xf;
      if ((uVar5 == 0) || ((int)uVar5 <= iVar2)) {
        uVar6 = 0;
        iVar2 = iVar2 - uVar5;
      }
      else {
        uVar6 = 1;
        iVar3 = FUN_00501510(param_1,uVar5 - 1,param_2);
        uVar7 = (uint)(iVar3 != 0);
      }
      if (uVar6 == 0) {
        uVar5 = (uint)param_1[0x19] >> 4 & 0xf;
        if ((uVar5 == 0) || ((int)uVar5 <= iVar2)) {
          uVar6 = 0;
          iVar2 = iVar2 - uVar5;
        }
        else {
          uVar6 = 1;
          iVar3 = FUN_005015a0(param_1,uVar5 - 1,param_2);
          if ((iVar3 == 0) || (uVar7 == 0)) {
            uVar7 = 0;
          }
          else {
            uVar7 = 1;
          }
        }
        if (uVar6 == 0) {
          uVar5 = (uint)param_1[0x19] >> 8 & 0xf;
          if ((uVar5 == 0) || ((int)uVar5 <= iVar2)) {
            uVar6 = 0;
            iVar2 = iVar2 - uVar5;
          }
          else {
            uVar6 = 1;
            iVar3 = FUN_00501640(param_1,uVar5 - 1,param_2);
            if ((iVar3 == 0) || (uVar7 == 0)) {
              uVar7 = 0;
            }
            else {
              uVar7 = 1;
            }
          }
          if (uVar6 == 0) {
            uVar5 = (uint)param_1[0x19] >> 0xc & 0xf;
            if ((uVar5 == 0) || ((int)uVar5 <= iVar2)) {
              uVar6 = 0;
              iVar2 = iVar2 - uVar5;
            }
            else {
              uVar6 = 1;
              iVar3 = FUN_005016e0(param_1,uVar5 - 1,param_2);
              if ((iVar3 == 0) || (uVar7 == 0)) {
                aiStack_8[1] = 0;
              }
              else {
                aiStack_8[1] = 1;
              }
            }
            if (uVar6 == 0) {
              uVar7 = (uint)param_1[0x19] >> 0x10 & 0xf;
              if ((uVar7 == 0) || ((int)uVar7 <= iVar2)) {
                uVar6 = 0;
                iVar2 = iVar2 - uVar7;
              }
              else {
                uVar6 = 1;
                FUN_00501780(param_1,uVar7 - 1,param_2);
              }
              if (uVar6 == 0) {
                uVar7 = (uint)param_1[0x19] >> 0x14 & 0xf;
                if ((uVar7 == 0) || ((int)uVar7 <= iVar2)) {
                  return 0;
                }
                uVar6 = 1;
                FUN_00501820(param_1,uVar7 - 1,param_2);
              }
            }
          }
        }
      }
    }
  }
  return uVar6;
}

