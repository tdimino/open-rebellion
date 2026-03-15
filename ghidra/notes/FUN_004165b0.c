
undefined4 FUN_004165b0(HINSTANCE param_1,int param_2,char *param_3,undefined4 param_4)

{
  char cVar1;
  bool bVar2;
  HANDLE pvVar3;
  DWORD DVar4;
  int iVar5;
  undefined3 extraout_var;
  void *this;
  void *pvVar6;
  undefined4 *this_00;
  uint uVar7;
  undefined4 unaff_EBX;
  uint uVar8;
  char *pcVar9;
  undefined4 auStack_18 [2];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062ad22;
  pvStack_c = ExceptionList;
  uVar8 = 0;
  ExceptionList = &pvStack_c;
  pvVar3 = CreateSemaphoreA((LPSECURITY_ATTRIBUTES)0x0,0,1,s_STARWARS_REBELLION_006a8314);
  DVar4 = GetLastError();
  if ((((pvVar3 == (HANDLE)0x0) || (DVar4 == 0xb7)) || (DVar4 == 6)) ||
     (iVar5 = FUN_0060a910(param_2,0x280,0x1e0,0x100), iVar5 == 0)) {
    ExceptionList = pvStack_c;
    return 1;
  }
  FUN_00416820();
  FUN_00406600();
  FUN_005f35a0();
  FUN_0060aa00();
  if (param_3 != (char *)0x0) {
    uVar7 = 0xffffffff;
    iVar5 = 0;
    pcVar9 = param_3;
    do {
      if (uVar7 == 0) break;
      uVar7 = uVar7 - 1;
      cVar1 = *pcVar9;
      pcVar9 = pcVar9 + 1;
    } while (cVar1 != '\0');
    if (0 < (int)(~uVar7 - 1)) {
      do {
        if (param_3[iVar5] == '-') {
          switch(param_3[iVar5 + 1]) {
          case 'L':
          case 'l':
            uVar8 = uVar8 | 0x100;
            break;
          case 'N':
          case 'n':
            uVar8 = uVar8 | 0x10;
            break;
          case 'W':
          case 'w':
            uVar8 = uVar8 | 1;
          }
          iVar5 = iVar5 + 1;
        }
        iVar5 = iVar5 + 1;
      } while (iVar5 < (int)(~uVar7 - 1));
    }
  }
  bVar2 = FUN_00416950();
  if (CONCAT31(extraout_var,bVar2) == 0) {
    FUN_004067e0();
    FUN_00597050();
    CloseHandle(pvVar3);
    ExceptionList = pvStack_c;
    return 0;
  }
  this = (void *)FUN_00618b70(0x160);
  uStack_4 = 0;
  if (this == (void *)0x0) {
    this_00 = (undefined4 *)0x0;
  }
  else {
    pvVar6 = FUN_005f35b0(auStack_18,param_3);
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    this_00 = (undefined4 *)FUN_00413000(this,param_1,uVar8 | 0x10,(int)pvVar6);
  }
  uStack_4 = 0;
  if (this != (void *)0x0) {
    FUN_005f2ff0(auStack_18);
  }
  uStack_4 = 0xffffffff;
  if (this_00 != (undefined4 *)0x0) {
    FUN_005fba00(this_00,0x280,0x1e0);
    pvVar3 = (HANDLE)FUN_00413560(this_00,param_4);
    (**(code **)*this_00)(1);
    CloseHandle(pvVar3);
    ExceptionList = pvStack_10;
    return unaff_EBX;
  }
  ExceptionList = pvStack_c;
  return 2;
}

