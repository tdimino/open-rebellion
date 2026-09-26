
void __fastcall FUN_005bab50(int *param_1)

{
  int *piVar1;
  int *piVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  DWORD DVar6;
  int iVar7;
  uint *puVar8;
  uint auStack_10 [4];
  
  piVar2 = (int *)(**(code **)(*param_1 + 0xc))();
joined_r0x005bab62:
  while (piVar1 = piVar2, piVar1 != (int *)0x0) {
    piVar2 = (int *)(**(code **)(*piVar1 + 4))();
    if (piVar1[0xc] == 0) goto LAB_005babaa;
    FUN_00610c80((uint)(piVar1 + 0xb));
    piVar1[8] = (uint)piVar1[0xf] >> 5 & 1;
    if (((*(byte *)(piVar1 + 0xf) & 0x20) == 0) &&
       (iVar3 = FUN_005f5500(param_1,piVar1[6]), iVar3 != 0)) {
      uVar4 = piVar1[6];
      goto LAB_005babbd;
    }
  }
  if (param_1[6] != 0) {
    uVar4 = 0;
    puVar8 = auStack_10;
    do {
      uVar4 = uVar4 + 1;
      uVar5 = FUN_005bad00(param_1,uVar4);
      *puVar8 = uVar5;
      puVar8 = puVar8 + 1;
    } while (uVar4 < 4);
    DVar6 = timeGetTime();
    piVar2 = (int *)thunk_FUN_005f5060((int)(param_1 + 3));
joined_r0x005bac08:
    piVar1 = piVar2;
    if (piVar1 != (int *)0x0) {
      piVar2 = (int *)(**(code **)(*piVar1 + 4))();
      iVar3 = param_1[piVar1[7] + 0x151];
      uVar4 = param_1[iVar3 + 0xf];
      iVar7 = FUN_005aaf90();
      if ((*(int *)(iVar7 + 0x8c8) != 0) && (iVar3 == 1)) {
        uVar4 = uVar4 << 2;
      }
      if (DVar6 - piVar1[9] <= uVar4) goto code_r0x005bac4b;
      FUN_005f54d0(param_1 + 3,piVar1[6]);
      goto joined_r0x005baca7;
    }
  }
  return;
LAB_005babaa:
  iVar3 = FUN_005f5500(param_1,piVar1[6]);
  if (iVar3 != 0) {
    uVar4 = piVar1[6];
LAB_005babbd:
    FUN_005f54a0(param_1,uVar4);
  }
  goto joined_r0x005bab62;
code_r0x005bac4b:
  if (auStack_10[iVar3 + -1] < (uint)param_1[iVar3 + 7]) {
    FUN_005f54d0(param_1 + 3,piVar1[6]);
    iVar7 = FUN_005bad50(param_1,(int)piVar1);
    if (iVar7 == 0) {
      iVar3 = FUN_005f5500(param_1,piVar1[6]);
      if (iVar3 == 0) {
joined_r0x005baca7:
        if (piVar1 != (int *)0x0) {
          (**(code **)*piVar1)(1);
        }
      }
    }
    else {
      auStack_10[iVar3 + -1] = auStack_10[iVar3 + -1] + 1;
      FUN_005f5440(param_1,piVar1);
    }
  }
  goto joined_r0x005bac08;
}

