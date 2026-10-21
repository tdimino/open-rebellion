
void __fastcall FUN_0059fca0(void *param_1)

{
  int iVar1;
  uint uVar2;
  uint *puVar3;
  uint local_10 [4];
  
  uVar2 = 0;
  puVar3 = local_10;
  do {
    iVar1 = FUN_0059f5f0(param_1,uVar2);
    *puVar3 = (uint)(iVar1 != 0);
    uVar2 = uVar2 + 1;
    puVar3 = puVar3 + 1;
  } while (uVar2 < 4);
  FUN_005c0e40((int *)local_10);
  return;
}

