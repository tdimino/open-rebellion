
int FUN_0045f7b0(void)

{
  char cVar1;
  LPCSTR pCVar2;
  LPBYTE pBVar3;
  void *pvVar4;
  char *pcVar5;
  int iVar6;
  uint uVar7;
  undefined4 *this;
  char *pcVar8;
  undefined4 local_3c [3];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630f2e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_3c);
  local_4 = 0;
  this = (undefined4 *)0x0;
  FUN_005f35b0(local_18,s_Star_Wars_Rebellion_006a7c50);
  local_4._0_1_ = 1;
  FUN_005f35b0(local_24,&DAT_006a7c4c);
  local_4._0_1_ = 2;
  FUN_005f35b0(local_30,s_SOFTWARE_LucasArts_Entertainment_006a7c1c);
  local_4._0_1_ = 3;
  FUN_005f30d0(local_3c,(int)local_30);
  FUN_005f30d0(local_3c,(int)local_24);
  FUN_005f30d0(local_3c,(int)local_18);
  local_4._0_1_ = 2;
  FUN_005f2ff0(local_30);
  local_4._0_1_ = 1;
  FUN_005f2ff0(local_24);
  local_4._0_1_ = 0;
  FUN_005f2ff0(local_18);
  pcVar8 = s_EncyLocation_006a8778;
  pCVar2 = (LPCSTR)FUN_00583c40((int)local_3c);
  pBVar3 = FUN_005f3fc0(pCVar2,pcVar8);
  if (pBVar3 != (LPBYTE)0x0) {
    pvVar4 = (void *)FUN_00618b70(0xc);
    local_4._0_1_ = 4;
    if (pvVar4 == (void *)0x0) {
      this = (undefined4 *)0x0;
    }
    else {
      this = (undefined4 *)FUN_005f35b0(pvVar4,(char *)pBVar3);
    }
    local_4._0_1_ = 0;
    FUN_00618b60(pBVar3);
  }
  if (this == (undefined4 *)0x0) {
    pvVar4 = (void *)FUN_00618b70(0xc);
    local_4._0_1_ = 5;
    if (pvVar4 == (void *)0x0) {
      this = (undefined4 *)0x0;
    }
    else {
      this = (undefined4 *)FUN_005f35b0(pvVar4,s_EDATA__006a8770);
    }
    local_4._0_1_ = 0;
    if (this == (undefined4 *)0x0) goto LAB_0045f934;
  }
  pcVar5 = (char *)FUN_00583c40((int)this);
  uVar7 = 0xffffffff;
  pcVar8 = pcVar5;
  do {
    if (uVar7 == 0) break;
    uVar7 = uVar7 - 1;
    cVar1 = *pcVar8;
    pcVar8 = pcVar8 + 1;
  } while (cVar1 != '\0');
  if (pcVar5[~uVar7 - 2] != '\\') {
    FUN_005f35b0(local_18,&DAT_006a7c4c);
    local_4._0_1_ = 6;
    FUN_005f30d0(this,(int)local_18);
    local_4._0_1_ = 0;
    FUN_005f2ff0(local_18);
  }
LAB_0045f934:
  iVar6 = FUN_005f3610((int)this);
  if (this != (undefined4 *)0x0) {
    (**(code **)*this)(1);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_3c);
  ExceptionList = local_c;
  return iVar6;
}

