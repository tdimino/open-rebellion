
void __thiscall FUN_004abe30(void *this,int *param_1)

{
  undefined4 *puVar1;
  uint *puVar2;
  
  puVar1 = *(undefined4 **)((int)this + 0x1c);
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar2 = FUN_004abe70(this,*(uint **)((int)this + 0x20),param_1);
  *(uint **)((int)this + 0x1c) = puVar2;
  return;
}

