
int __fastcall FUN_00613d10(LPVOID param_1)

{
  HANDLE pvVar1;
  int iVar2;
  undefined4 *puVar3;
  int local_18;
  DWORD local_14;
  uint local_10;
  undefined4 local_c;
  int local_8;
  undefined4 local_4;
  
  local_18 = 0;
  if ((*(int *)((int)param_1 + 0x34) != 0) && (*(int *)((int)param_1 + 0x38) != 0)) {
    puVar3 = (undefined4 *)((int)param_1 + 0x40);
    iVar2 = 3;
    do {
      pvVar1 = CreateEventA((LPSECURITY_ATTRIBUTES)0x0,0,0,(LPCSTR)0x0);
      *puVar3 = pvVar1;
      if (pvVar1 == (HANDLE)0x0) {
        local_18 = -0x7fffbffb;
      }
      puVar3 = puVar3 + 1;
      iVar2 = iVar2 + -1;
    } while (iVar2 != 0);
    if (local_18 == 0) {
      local_c = *(undefined4 *)((int)param_1 + 0x40);
      local_10 = *(uint *)((int)param_1 + 0x50) >> 1;
      local_8 = *(uint *)((int)param_1 + 0x50) - 1;
      local_4 = *(undefined4 *)((int)param_1 + 0x44);
      pvVar1 = CreateThread((LPSECURITY_ATTRIBUTES)0x0,0x4000,lpStartAddress_006136b0,param_1,0,
                            &local_14);
      *(HANDLE *)((int)param_1 + 0x3c) = pvVar1;
      if (pvVar1 == (HANDLE)0x0) {
        local_18 = -0x7fffbffb;
      }
      if (local_18 == 0) {
        local_18 = (**(code **)(**(int **)((int)param_1 + 0x38) + 0xc))
                             (*(int **)((int)param_1 + 0x38),2,&local_10);
        SetThreadPriority(*(HANDLE *)((int)param_1 + 0x3c),1);
        if (local_18 == 0) {
          return 0;
        }
      }
    }
    FUN_00613e00((int)param_1);
    return local_18;
  }
  return -0x7fffbffb;
}

