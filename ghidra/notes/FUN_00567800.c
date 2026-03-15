
bool __thiscall FUN_00567800(void *this,int *param_1)

{
  int iVar1;
  void *this_00;
  undefined4 *puVar2;
  bool bVar3;
  uint uStack_14;
  
  uStack_14 = 0x567811;
  (**(code **)(*param_1 + 4))();
  bVar3 = *(int *)((int)this + 8) != 0;
  if (bVar3) {
    uStack_14 = 0x567826;
    iVar1 = thunk_FUN_005f5060(*(int *)((int)this + 8));
    while ((iVar1 != 0 && (bVar3))) {
      uStack_14 = 0x567838;
      this_00 = (void *)thunk_FUN_005f5060(iVar1 + 0x30);
      while ((this_00 != (void *)0x0 && (bVar3))) {
        FUN_00402e40(this_00,&uStack_14);
        puVar2 = FUN_004f42a0(param_1);
        this_00 = *(void **)((int)this_00 + 0x10);
        bVar3 = puVar2 != (undefined4 *)0x0;
      }
      iVar1 = *(int *)(iVar1 + 0x10);
    }
  }
  return bVar3;
}

