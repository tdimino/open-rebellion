
undefined4 __fastcall FUN_004b8ab0(void *param_1)

{
  int iVar1;
  void *pvVar2;
  int iVar3;
  undefined4 uStack_4;
  
  uStack_4 = 0;
  iVar1 = FUN_00476110(*(void **)((int)param_1 + 0xc),(undefined4 *)((int)param_1 + 0x28));
  if ((iVar1 != 0) && (0 < *(int *)(iVar1 + 0x80))) {
    pvVar2 = (void *)FUN_0047aa90(*(void **)((int)param_1 + 0xc),(undefined4 *)((int)param_1 + 0x5c)
                                 );
    if (pvVar2 != (void *)0x0) {
      iVar3 = FUN_004ae240(param_1,iVar1,pvVar2);
      if (iVar3 != 0) {
        uStack_4 = 1;
        *(uint *)((int)pvVar2 + 0x30) = *(uint *)((int)pvVar2 + 0x30) | 0x200000;
        FUN_004f26d0((void *)(iVar1 + 0x2c),(undefined4 *)((int)param_1 + 0x5c));
        *(uint *)(iVar1 + 0x38) = *(uint *)(iVar1 + 0x38) | 0x20000000;
      }
    }
  }
  return uStack_4;
}

