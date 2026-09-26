
int __fastcall FUN_005f92e0(void *param_1)

{
  bool bVar1;
  DWORD DVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  int iVar6;
  uint uVar7;
  
  iVar6 = 0;
  DVar2 = timeGetTime();
  bVar1 = false;
  iVar3 = thunk_FUN_005f5060((int)param_1);
  while( true ) {
    if ((iVar3 == 0) || (*(int *)((int)param_1 + 0x108) == 0)) {
      if (!bVar1) {
        iVar6 = 0xe;
      }
      return iVar6;
    }
    if (*(char *)(iVar3 + 0x26) != '\0') {
      uVar7 = 0;
      iVar4 = FUN_005f8a20((int)param_1);
      if (iVar4 != 0) {
        do {
          FUN_005f88c0((int)param_1);
          iVar4 = FUN_005fb390(*(void **)(iVar3 + 0x1c),uVar7);
          if (iVar4 == 1) {
            iVar4 = FUN_005fb3a0(*(void **)(iVar3 + 0x1c),uVar7);
            bVar1 = true;
            if (*(uint *)((int)param_1 + 0x194) < DVar2 - iVar4) {
              iVar6 = FUN_005f9090(param_1,iVar3,uVar7,0);
              FUN_005f9030(param_1);
              if (iVar6 != 0) {
                FUN_005ae1e0();
                return iVar6;
              }
            }
          }
          uVar7 = uVar7 + 1;
          uVar5 = FUN_005f8a20((int)param_1);
        } while (uVar7 < uVar5);
      }
    }
    if (*(int *)((int)param_1 + 0x108) == 0) break;
    iVar3 = *(int *)(iVar3 + 0x10);
  }
  return 1;
}

