
DWORD __fastcall FUN_0060af70(int param_1)

{
  HMODULE hModule;
  HRSRC hResInfo;
  undefined2 *puVar1;
  void *pvVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  undefined2 extraout_var;
  undefined2 extraout_var_00;
  undefined2 extraout_var_01;
  undefined2 extraout_var_02;
  uint uVar6;
  undefined2 extraout_var_03;
  undefined2 extraout_var_04;
  undefined2 extraout_var_05;
  undefined2 extraout_var_06;
  uint uVar7;
  int iVar8;
  undefined4 uVar9;
  uint *puVar10;
  int iVar11;
  int iVar12;
  HGLOBAL local_18;
  DWORD local_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656978;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  hModule = (HMODULE)FUN_005fefd0((uint)*(ushort *)(param_1 + 0xe8));
  local_14 = 0;
  local_18 = (HGLOBAL)0x0;
  if (hModule != (HMODULE)0x0) {
    hResInfo = FindResourceA(hModule,(LPCSTR)(uint)*(ushort *)(param_1 + 0xea),(LPCSTR)0xa);
    if (hResInfo == (HRSRC)0x0) {
      local_14 = GetLastError();
    }
    else {
      local_18 = LoadResource(hModule,hResInfo);
    }
  }
  if (local_18 != (HGLOBAL)0x0) {
    puVar1 = LockResource(local_18);
    *(undefined4 *)(param_1 + 0xc4) = 0;
    *(undefined4 *)(param_1 + 0xcc) = 0;
    *(undefined4 *)(param_1 + 0xd0) = 0;
    *(undefined2 *)(param_1 + 0xac) = *puVar1;
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_005fbd20(pvVar2,hModule,CONCAT22(extraout_var,puVar1[1]),2);
    }
    uStack_4 = 0xffffffff;
    *(void **)(param_1 + 0xb4) = pvVar2;
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4 = 1;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_005fbd20(pvVar2,hModule,CONCAT22(extraout_var_05,puVar1[6]),2);
    }
    uStack_4 = 0xffffffff;
    *(void **)(param_1 + 0xb8) = pvVar2;
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4 = 2;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_005fbd20(pvVar2,hModule,CONCAT22(extraout_var_00,puVar1[7]),2);
    }
    uStack_4 = 0xffffffff;
    *(void **)(param_1 + 0xbc) = pvVar2;
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4 = 3;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_005fbd20(pvVar2,hModule,CONCAT22(extraout_var_06,puVar1[8]),2);
    }
    uStack_4 = 0xffffffff;
    *(void **)(param_1 + 0xc0) = pvVar2;
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4 = 4;
    if (pvVar2 == (void *)0x0) {
      piVar3 = (int *)0x0;
    }
    else {
      piVar3 = FUN_005fbd20(pvVar2,hModule,CONCAT22(extraout_var_01,puVar1[2]),2);
    }
    uStack_4 = 0xffffffff;
    if (piVar3 != (int *)0x0) {
      pvVar2 = (void *)FUN_00618b70(0x114);
      uStack_4 = 5;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        iVar12 = *(int *)(param_1 + 0xec);
        puVar10 = *(uint **)(param_1 + 0xb4);
        uVar6 = CONCAT22(extraout_var_02,puVar1[3]);
        uVar7 = CONCAT22((short)((uint)iVar12 >> 0x10),puVar1[2]);
        iVar11 = 0x32;
        uVar9 = 100;
        iVar8 = param_1;
        iVar4 = FUN_005fc0f0(piVar3);
        iVar5 = FUN_005fc0e0(piVar3);
        pvVar2 = FUN_00602150(pvVar2,hModule,0,0,iVar5,iVar4,iVar8,uVar9,uVar7,uVar6,puVar10,iVar11,
                              iVar12);
      }
      uStack_4 = 0xffffffff;
      *(void **)(param_1 + 0xd4) = pvVar2;
      FUN_005fbfa0(piVar3);
      FUN_00618b60((undefined *)piVar3);
    }
    if (*(void **)(param_1 + 0xd4) != (void *)0x0) {
      FUN_005ffce0(*(void **)(param_1 + 0xd4),0);
      FUN_005f4f10((void *)(param_1 + 0x6c),*(int *)(param_1 + 0xd4));
    }
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4 = 6;
    if (pvVar2 == (void *)0x0) {
      piVar3 = (int *)0x0;
    }
    else {
      piVar3 = FUN_005fbd20(pvVar2,hModule,CONCAT22(extraout_var_03,puVar1[4]),2);
    }
    uStack_4 = 0xffffffff;
    if (piVar3 != (int *)0x0) {
      pvVar2 = (void *)FUN_00618b70(0x114);
      uStack_4 = 7;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        iVar12 = *(int *)(param_1 + 0xec);
        puVar10 = *(uint **)(param_1 + 0xb4);
        uVar6 = CONCAT22(extraout_var_04,puVar1[5]);
        uVar7 = CONCAT22((short)((uint)iVar12 >> 0x10),puVar1[4]);
        iVar11 = 0x32;
        uVar9 = 0x65;
        iVar8 = param_1;
        iVar4 = FUN_005fc0f0(piVar3);
        iVar5 = FUN_005fc0e0(piVar3);
        pvVar2 = FUN_00602150(pvVar2,hModule,0,0,iVar5,iVar4,iVar8,uVar9,uVar7,uVar6,puVar10,iVar11,
                              iVar12);
      }
      uStack_4 = 0xffffffff;
      *(void **)(param_1 + 0xd8) = pvVar2;
      FUN_005fbfa0(piVar3);
      FUN_00618b60((undefined *)piVar3);
    }
    if (*(void **)(param_1 + 0xd8) != (void *)0x0) {
      FUN_005ffce0(*(void **)(param_1 + 0xd8),0);
      FUN_005f4f10((void *)(param_1 + 0x6c),*(int *)(param_1 + 0xd8));
    }
  }
  FreeResource(local_18);
  ExceptionList = pvStack_c;
  return local_14;
}

