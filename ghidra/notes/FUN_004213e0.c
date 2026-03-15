
void * __thiscall
FUN_004213e0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5,int param_6,int param_7,undefined4 param_8,int param_9)

{
  char cVar1;
  int iVar2;
  HGDIOBJ pvVar3;
  void *pvVar4;
  LPCSTR pCVar5;
  LPBYTE pBVar6;
  undefined4 *puVar7;
  uint uVar8;
  undefined4 uVar9;
  int iVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  char *pcVar13;
  uint uVar14;
  char *pcVar15;
  int iStack_d4;
  int aiStack_d0 [2];
  undefined1 auStack_c8 [8];
  undefined4 auStack_c0 [8];
  int iStack_a0;
  _RTL_CRITICAL_SECTION _Stack_9c;
  int iStack_70;
  int aiStack_6c [2];
  undefined1 auStack_64 [8];
  undefined4 auStack_5c [8];
  int iStack_3c;
  _RTL_CRITICAL_SECTION _Stack_38;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062ba6f;
  pvStack_c = ExceptionList;
  uVar14 = 0;
  uVar12 = 0;
  uVar11 = 0;
  iVar10 = 0;
  uVar9 = 0;
  ExceptionList = &pvStack_c;
  pvVar3 = GetStockObject(4);
  FUN_005ff910(this,param_3,s_MOVIE_WINDOW_006a85a8,0x40000000,param_4,param_5,param_6,param_7,
               param_8,(char *)0x0,pvVar3,uVar9,iVar10,uVar11,uVar12,uVar14);
  uStack_4 = 0;
  FUN_005f2f50((undefined4 *)((int)this + 0xa4));
  *(undefined4 *)((int)this + 0x98) = param_1;
  uStack_4._0_1_ = 1;
  *(undefined ***)this = &PTR_FUN_00658bb0;
  *(undefined4 *)((int)this + 0x94) = 0xf;
  *(undefined4 *)((int)this + 0xb4) = param_2;
  *(undefined4 *)((int)this + 0xb0) = 0;
  *(undefined4 *)((int)this + 0x9c) = 0;
  *(undefined4 *)((int)this + 0xa0) = 0;
  if (param_9 != 0) {
    FUN_005f3090((undefined4 *)((int)this + 0xa4),param_9);
  }
  FUN_00615db0(&iStack_70,1);
  uStack_4._0_1_ = 2;
  FUN_00615db0(&iStack_d4,1);
  pcVar15 = s_Star_Wars_Rebellion_006a7c50;
  pcVar13 = &DAT_006a7c4c;
  uStack_4 = CONCAT31(uStack_4._1_3_,3);
  pvVar4 = FUN_00616110(auStack_64,s_SOFTWARE_LucasArts_Entertainment_006a7c1c);
  pvVar4 = FUN_00616110(pvVar4,pcVar13);
  pvVar4 = FUN_00616110(pvVar4,pcVar15);
  FUN_00615f00(pvVar4,0);
  pcVar13 = s_TooBigLocation_006a7c74;
  pCVar5 = (LPCSTR)FUN_00615680(*(int *)((int)aiStack_6c + *(int *)(iStack_70 + 4)));
  pBVar6 = FUN_005f3fc0(pCVar5,pcVar13);
  FUN_00615660(*(void **)((int)aiStack_6c + *(int *)(iStack_70 + 4)),0);
  iVar10 = *(int *)(iStack_70 + 4);
  if (*(int *)((int)&iStack_3c + iVar10) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)((int)&_Stack_38.DebugInfo + iVar10));
  }
  iVar2 = *(int *)((int)&iStack_3c + iVar10);
  *(undefined4 *)((int)aiStack_6c + iVar10 + 4) = 0;
  if (iVar2 < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)((int)&_Stack_38.DebugInfo + iVar10));
  }
  if (pBVar6 != (LPBYTE)0x0) {
    pcVar13 = &DAT_006a7c4c;
    pvVar4 = FUN_00616110(auStack_c8,(char *)pBVar6);
    FUN_00616110(pvVar4,pcVar13);
    FUN_00618b60(pBVar6);
  }
  pcVar13 = FUN_00421ba0((int)this);
  pvVar4 = FUN_00616110(auStack_c8,pcVar13);
  FUN_00615f00(pvVar4,0);
  pcVar13 = (char *)FUN_00615680(*(int *)((int)aiStack_d0 + *(int *)(iStack_d4 + 4)));
  if (pcVar13 != (char *)0x0) {
    uVar14 = 0xffffffff;
    pcVar15 = pcVar13;
    do {
      if (uVar14 == 0) break;
      uVar14 = uVar14 - 1;
      cVar1 = *pcVar15;
      pcVar15 = pcVar15 + 1;
    } while (cVar1 != '\0');
    uVar8 = ~uVar14 - 1;
    puVar7 = (undefined4 *)FUN_00618b70(~uVar14);
    *(undefined4 **)((int)this + 0x9c) = puVar7;
    if (puVar7 != (undefined4 *)0x0) {
      for (uVar14 = uVar8 >> 2; uVar14 != 0; uVar14 = uVar14 - 1) {
        *puVar7 = 0;
        puVar7 = puVar7 + 1;
      }
      for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *(undefined1 *)puVar7 = 0;
        puVar7 = (undefined4 *)((int)puVar7 + 1);
      }
      uVar14 = 0xffffffff;
      do {
        pcVar15 = pcVar13;
        if (uVar14 == 0) break;
        uVar14 = uVar14 - 1;
        pcVar15 = pcVar13 + 1;
        cVar1 = *pcVar13;
        pcVar13 = pcVar15;
      } while (cVar1 != '\0');
      uVar14 = ~uVar14;
      pcVar13 = pcVar15 + -uVar14;
      pcVar15 = *(char **)((int)this + 0x9c);
      for (uVar8 = uVar14 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
        *(undefined4 *)pcVar15 = *(undefined4 *)pcVar13;
        pcVar13 = pcVar13 + 4;
        pcVar15 = pcVar15 + 4;
      }
      for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
        *pcVar15 = *pcVar13;
        pcVar13 = pcVar13 + 1;
        pcVar15 = pcVar15 + 1;
      }
    }
  }
  FUN_00615660(*(void **)((int)aiStack_d0 + *(int *)(iStack_d4 + 4)),0);
  iVar10 = *(int *)(iStack_d4 + 4);
  if (*(int *)((int)&iStack_a0 + iVar10) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)((int)&_Stack_9c.DebugInfo + iVar10));
  }
  iVar2 = *(int *)((int)&iStack_a0 + iVar10);
  *(undefined4 *)((int)aiStack_d0 + iVar10 + 4) = 0;
  if (iVar2 < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)((int)&_Stack_9c.DebugInfo + iVar10));
  }
  uStack_4._0_1_ = 2;
  FUN_00615ea0((int)auStack_c0);
  FUN_00615440(auStack_c0);
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  FUN_00615ea0((int)auStack_5c);
  FUN_00615440(auStack_5c);
  ExceptionList = pvStack_c;
  return this;
}

