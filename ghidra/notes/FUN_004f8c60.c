
/* WARNING: Removing unreachable block (ram,0x004f8da0) */

undefined4 __thiscall
FUN_004f8c60(void *this,char *param_1,char *param_2,char *param_3,char *param_4,char *param_5,
            int param_6)

{
  int iVar1;
  uint *puVar2;
  uint uVar3;
  void *pvVar4;
  int *piVar5;
  int *piVar6;
  undefined *puVar7;
  int iVar8;
  char *pcVar9;
  char *pcVar10;
  char *pcVar11;
  LPCSTR *ppCVar12;
  LPCSTR *ppCVar13;
  char *pcVar14;
  char *pcVar15;
  char *local_1c;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063feb8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar1 = FUN_006158b0();
  if (iVar1 != 0) {
    if ((DAT_006b296c & 1) == 0) {
      DAT_006b296c = DAT_006b296c | 1;
      FUN_00615c90(&DAT_006b2a70,&DAT_006b2970,0xf9,2,1);
      FUN_00618c20(0x4f9170);
    }
    iVar1 = *(int *)(DAT_006b2a70 + 4);
    iVar8 = (int)&DAT_006b2a70 + iVar1;
    FUN_004f90b0(iVar8);
    *(undefined4 *)(&DAT_006b2a78 + iVar1) = 0;
    FUN_004f90d0(iVar8);
    FUN_006169a0(&DAT_006b2a70,0,0);
    DAT_006b2a69 = 0;
    puVar2 = FUN_004025b0(this,&local_10);
    local_4 = 0;
    pcVar9 = s_Alliance_006a8f04;
    uVar3 = *(uint *)((int)this + 0x24) >> 6 & 3;
    if (uVar3 == 1) {
      local_1c = s_Alliance_006a8f04;
    }
    else if (uVar3 == 2) {
      local_1c = s_Empire_006a8f10;
    }
    else if (uVar3 == 3) {
      local_1c = s_Neutral_006a8f1c;
    }
    else {
      local_1c = s_UNKNOWN__006a8f28;
    }
    iVar1 = *(int *)(param_6 + 0x14);
    uVar3 = *(uint *)((int)this + 0x24) >> 4 & 3;
    if (uVar3 != 1) {
      if (uVar3 == 2) {
        pcVar9 = s_Empire_006a8f10;
      }
      else {
        pcVar9 = s_UNKNOWN__006a8f28;
      }
    }
    FUN_00441a50((int)&DAT_006b2a70 + *(int *)(DAT_006b2a70 + 4),4);
    FUN_004f90f0((int)&DAT_006b2a70 + *(int *)(DAT_006b2a70 + 4),2);
    ppCVar13 = &lpString_006a7f40;
    ppCVar12 = &lpString_006a7f40;
    iVar8 = FUN_004fd340();
    pvVar4 = FUN_00616e10(&DAT_006b2a70,iVar8);
    pvVar4 = FUN_00616110(pvVar4,(char *)ppCVar12);
    piVar5 = (int *)FUN_00616110(pvVar4,(char *)ppCVar13);
    piVar6 = piVar5;
    if (piVar5 != (int *)0x0) {
      piVar6 = (int *)(*(int *)(*piVar5 + 4) + (int)piVar5);
    }
    FUN_00441a50((int)piVar6,7);
    if (piVar5 == (int *)0x0) {
      iVar8 = 0;
    }
    else {
      iVar8 = *(int *)(*piVar5 + 4) + (int)piVar5;
    }
    FUN_004f9130(iVar8,2);
    ppCVar12 = &lpString_006a7f40;
    pvVar4 = FUN_00616110(piVar5,pcVar9);
    piVar5 = (int *)FUN_00616110(pvVar4,(char *)ppCVar12);
    piVar6 = piVar5;
    if (piVar5 != (int *)0x0) {
      piVar6 = (int *)(*(int *)(*piVar5 + 4) + (int)piVar5);
    }
    FUN_00441a50((int)piVar6,0x28);
    if (piVar5 == (int *)0x0) {
      iVar8 = 0;
    }
    else {
      iVar8 = *(int *)(*piVar5 + 4) + (int)piVar5;
    }
    FUN_004f9130(iVar8,2);
    pcVar9 = &DAT_006a8f50;
    pvVar4 = FUN_00616110(piVar5,param_1);
    piVar5 = (int *)FUN_00616110(pvVar4,pcVar9);
    piVar6 = piVar5;
    if (piVar5 != (int *)0x0) {
      piVar6 = (int *)(*(int *)(*piVar5 + 4) + (int)piVar5);
    }
    FUN_00441a50((int)piVar6,4);
    if (piVar5 == (int *)0x0) {
      iVar8 = 0;
    }
    else {
      iVar8 = *(int *)(*piVar5 + 4) + (int)piVar5;
    }
    FUN_004f90f0(iVar8,2);
    ppCVar12 = &lpString_006a7f40;
    pcVar9 = &DAT_006a8f4c;
    pvVar4 = FUN_00617480(piVar5,iVar1);
    pvVar4 = FUN_00616110(pvVar4,pcVar9);
    piVar5 = (int *)FUN_00616110(pvVar4,(char *)ppCVar12);
    piVar6 = piVar5;
    if (piVar5 != (int *)0x0) {
      piVar6 = (int *)(*(int *)(*piVar5 + 4) + (int)piVar5);
    }
    FUN_00441a50((int)piVar6,7);
    if (piVar5 == (int *)0x0) {
      iVar1 = 0;
    }
    else {
      iVar1 = *(int *)(*piVar5 + 4) + (int)piVar5;
    }
    FUN_004f9130(iVar1,2);
    ppCVar12 = &lpString_006a7f40;
    pvVar4 = FUN_00616110(piVar5,local_1c);
    piVar5 = (int *)FUN_00616110(pvVar4,(char *)ppCVar12);
    piVar6 = piVar5;
    if (piVar5 != (int *)0x0) {
      piVar6 = (int *)(*(int *)(*piVar5 + 4) + (int)piVar5);
    }
    FUN_00441a50((int)piVar6,0x1e);
    if (piVar5 == (int *)0x0) {
      iVar1 = 0;
    }
    else {
      iVar1 = *(int *)(*piVar5 + 4) + (int)piVar5;
    }
    FUN_004f9130(iVar1,2);
    pcVar14 = &DAT_006a8f50;
    ppCVar12 = &lpString_006a7f40;
    puVar7 = FUN_004f62d0((int)this);
    pcVar9 = (char *)FUN_00583c40((int)puVar7);
    pvVar4 = FUN_00616110(piVar5,pcVar9);
    pvVar4 = FUN_00616110(pvVar4,(char *)ppCVar12);
    piVar5 = (int *)FUN_00616110(pvVar4,pcVar14);
    piVar6 = piVar5;
    if (piVar5 != (int *)0x0) {
      piVar6 = (int *)(*(int *)(*piVar5 + 4) + (int)piVar5);
    }
    FUN_00441a50((int)piVar6,4);
    if (piVar5 == (int *)0x0) {
      iVar1 = 0;
    }
    else {
      iVar1 = *(int *)(*piVar5 + 4) + (int)piVar5;
    }
    FUN_004f90f0(iVar1,2);
    pcVar9 = &DAT_006a8f4c;
    pvVar4 = FUN_00617480(piVar5,*puVar2 & 0xffffff);
    piVar5 = (int *)FUN_00616110(pvVar4,pcVar9);
    piVar6 = piVar5;
    if (piVar5 != (int *)0x0) {
      piVar6 = (int *)(*(int *)(*piVar5 + 4) + (int)piVar5);
    }
    FUN_004f9130((int)piVar6,2);
    pcVar15 = &DAT_006a8f48;
    pcVar11 = s_Old__006a8f40;
    pcVar10 = &DAT_006a8f3c;
    pcVar14 = s_New__006a8f34;
    pcVar9 = &DAT_006a872c;
    pvVar4 = FUN_00616110(piVar5,(char *)&lpString_006a7f40);
    pvVar4 = FUN_00616110(pvVar4,param_2);
    pvVar4 = FUN_00616110(pvVar4,pcVar9);
    pvVar4 = FUN_00616110(pvVar4,pcVar14);
    pvVar4 = FUN_00616110(pvVar4,param_3);
    pvVar4 = FUN_00616110(pvVar4,pcVar10);
    pvVar4 = FUN_00616110(pvVar4,pcVar11);
    pvVar4 = FUN_00616110(pvVar4,param_4);
    FUN_00616110(pvVar4,pcVar15);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (param_5 != (char *)0x0) {
      pvVar4 = FUN_00616110(&DAT_006b2a70,(char *)&lpString_006a7f40);
      FUN_00616110(pvVar4,param_5);
    }
    FUN_00615f00(&DAT_006b2a70,0);
    FUN_00616140(&DAT_006b2a70);
  }
  ExceptionList = local_c;
  return 1;
}

