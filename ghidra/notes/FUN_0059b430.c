
void __fastcall FUN_0059b430(int param_1)

{
  undefined4 *puVar1;
  int iVar2;
  
  puVar1 = (undefined4 *)(param_1 + 0xb8);
  iVar2 = 8;
  do {
    FUN_006030f0((void *)*puVar1,5);
    puVar1 = puVar1 + 1;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  return;
}

