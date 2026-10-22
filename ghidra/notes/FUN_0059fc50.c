
void __thiscall FUN_0059fc50(void *this,uint param_1)

{
  int iVar1;
  uint uVar2;
  uint *puVar3;
  uint local_20 [8];
  
  uVar2 = 0;
  puVar3 = local_20;
  do {
    iVar1 = FUN_0059f5d0(this,uVar2);
    *puVar3 = (uint)(iVar1 != 0);
    uVar2 = uVar2 + 1;
    puVar3 = puVar3 + 1;
  } while (uVar2 < 8);
  FUN_005c0e00((int *)local_20,param_1);
  return;
}

