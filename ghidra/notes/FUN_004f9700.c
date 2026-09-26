
uint __fastcall FUN_004f9700(int *param_1)

{
  int iVar1;
  uint uVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar3;
  uint uVar4;
  bool bVar5;
  int *piStack_4;
  
  piStack_4 = param_1;
  iVar1 = FUN_0053a1c0((int)param_1);
  uVar4 = 0;
  if (iVar1 != 0) {
    if ((param_1[9] & 0x30U) == 0) {
      bVar5 = param_1[0x15] != 0;
    }
    else {
      bVar5 = param_1[0x15] == 0;
    }
    uVar2 = (uint)bVar5;
    uVar4 = 0;
    if (uVar2 != 0) {
      if ((param_1[9] & 0x30U) == 0) {
        uVar2 = (**(code **)(*(int *)param_1[0x15] + 0x10))();
      }
      uVar4 = 0;
      if (uVar2 != 0) {
        if ((param_1[0x14] & 0x20U) != 0) {
          uVar2 = (uint)param_1[0x14] >> 0xb & 1;
        }
        uVar4 = 0;
        if (uVar2 != 0) {
          if ((param_1[0x14] & 0x800U) != 0) {
            uVar2 = (uint)param_1[0x14] >> 5 & 1;
          }
          uVar4 = 0;
          if (uVar2 != 0) {
            if ((param_1[0x14] & 0x20U) != 0) {
              uVar2 = (uint)param_1[0x14] >> 4 & 1;
            }
            uVar4 = 0;
            if (uVar2 != 0) {
              bVar5 = FUN_005406d0(param_1);
              if (CONCAT31(extraout_var,bVar5) != 0) {
                piStack_4 = (int *)0x0;
                bVar5 = FUN_004f6b50(param_1,(int *)&piStack_4);
                uVar2 = 0;
                if (CONCAT31(extraout_var_00,bVar5) != 0) {
                  uVar2 = (uint)piStack_4[0x14] >> 1 & 1;
                }
                if (uVar2 == 0) {
                  return 0;
                }
                if (((param_1[0x14] & 0x10U) != 0) && ((param_1[0x14] & 0x20U) == 0)) {
                  uVar2 = (uint)piStack_4[0x14] >> 4 & 1;
                }
              }
              uVar4 = 0;
              if ((uVar2 != 0) && (uVar4 = 0, ((uint)~param_1[0x14] >> 0xc & 1) != 0)) {
                if ((*(byte *)(param_1 + 0x14) & 0x10) == 0) {
                  bVar5 = param_1[0x11] == 0;
                }
                else {
                  iVar1 = param_1[0x11];
                  if (iVar1 < 1) {
                    return 0;
                  }
                  iVar3 = FUN_004fd340();
                  bVar5 = iVar3 <= iVar1;
                }
                uVar4 = 0;
                if (bVar5) {
                  uVar4 = param_1[0x14];
                  uVar2 = FUN_004ece60((uint *)(param_1 + 0xf));
                  uVar4 = (uint)(uVar2 == (uVar4 >> 4 & 1));
                }
              }
            }
          }
        }
      }
    }
  }
  return uVar4;
}

