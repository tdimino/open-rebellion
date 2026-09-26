
uint __cdecl FUN_00619030(char *param_1,uint param_2,uint param_3,int *param_4)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  uint uVar5;
  int *piVar6;
  int *piVar7;
  char *pcVar8;
  int *piVar9;
  char *pcVar10;
  
  piVar2 = param_4;
  piVar6 = (int *)(param_3 * param_2);
  if (piVar6 == (int *)0x0) {
    return 0;
  }
  piVar7 = piVar6;
  if ((param_4[3] & 0x10cU) == 0) {
    param_4 = (int *)0x1000;
    piVar9 = (int *)0x1000;
  }
  else {
    piVar9 = (int *)param_4[6];
    param_4 = piVar9;
  }
  do {
    if (((piVar2[3] & 0x10cU) == 0) || (piVar3 = (int *)piVar2[1], piVar3 == (int *)0x0)) {
      if (piVar7 < piVar9) {
        uVar5 = FUN_0061ef70(piVar2);
        if (uVar5 == 0xffffffff) {
          return (uint)((int)piVar6 - (int)piVar7) / param_2;
        }
        *param_1 = (char)uVar5;
        piVar9 = (int *)piVar2[6];
        param_1 = param_1 + 1;
        iVar1 = -1;
        param_4 = piVar9;
      }
      else {
        piVar3 = piVar7;
        if (piVar9 != (int *)0x0) {
          piVar3 = (int *)((int)piVar7 - (uint)piVar7 % (uint)piVar9);
        }
        iVar4 = FUN_0061b8d0(piVar2[4],param_1,(DWORD)piVar3);
        if (iVar4 == 0) {
          piVar2[3] = piVar2[3] | 0x10;
          return (uint)((int)piVar6 - (int)piVar7) / param_2;
        }
        if (iVar4 == -1) {
          piVar2[3] = piVar2[3] | 0x20;
          return (uint)((int)piVar6 - (int)piVar7) / param_2;
        }
        iVar1 = -iVar4;
        param_1 = param_1 + iVar4;
      }
    }
    else {
      if (piVar7 < piVar3) {
        piVar3 = piVar7;
      }
      iVar1 = -(int)piVar3;
      pcVar8 = (char *)*piVar2;
      pcVar10 = param_1;
      for (uVar5 = (uint)piVar3 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        *(undefined4 *)pcVar10 = *(undefined4 *)pcVar8;
        pcVar8 = pcVar8 + 4;
        pcVar10 = pcVar10 + 4;
      }
      for (uVar5 = (uint)piVar3 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar10 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar10 = pcVar10 + 1;
      }
      param_1 = param_1 + (int)piVar3;
      piVar2[1] = piVar2[1] - (int)piVar3;
      *piVar2 = *piVar2 + (int)piVar3;
      piVar9 = param_4;
    }
    piVar7 = (int *)((int)piVar7 + iVar1);
    if (piVar7 == (int *)0x0) {
      return param_3;
    }
  } while( true );
}

