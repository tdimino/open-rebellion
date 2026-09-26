
undefined4 __thiscall FUN_005f9790(void *this,uint param_1)

{
  uint *puVar1;
  int iVar2;
  int iVar3;
  
  if ((param_1 < DAT_006ac294) && (*(int *)((int)this + param_1 * 4 + 0x34) != -1)) {
    *(undefined4 *)((int)this + param_1 * 4 + 0x34) = 0xffffffff;
    *(int *)((int)this + 0x94) = *(int *)((int)this + 0x94) + -1;
    iVar2 = thunk_FUN_005f5060((int)this);
    while (iVar2 != 0) {
      iVar3 = FUN_005fb390(*(void **)(iVar2 + 0x1c),param_1);
      if (iVar3 == 1) {
        FUN_005fb320(*(void **)(iVar2 + 0x1c),param_1,2);
      }
      iVar3 = FUN_005fb340(*(int *)(iVar2 + 0x1c));
      if (iVar3 == 1) {
        puVar1 = (uint *)(iVar2 + 0x18);
        iVar2 = *(int *)(iVar2 + 0x10);
        FUN_005f54a0(this,*puVar1);
      }
      else {
        iVar2 = *(int *)(iVar2 + 0x10);
      }
    }
    if ((param_1 != 0xffffffff) && (param_1 != 0xf)) {
      FUN_005f8600(this);
      FUN_005ae1e0();
      PostMessageA(*(HWND *)((int)this + 0x2c),0x40c,1,param_1);
    }
    return 0;
  }
  return 0xb;
}

