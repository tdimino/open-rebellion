
void FUN_005969d0(undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  undefined1 *puVar3;
  undefined1 local_1c [28];
  
  puVar3 = local_1c;
  FUN_00597530(local_1c);
  puVar1 = (undefined4 *)FUN_0059a730(puVar3);
  for (iVar2 = 7; iVar2 != 0; iVar2 = iVar2 + -1) {
    *param_1 = *puVar1;
    puVar1 = puVar1 + 1;
    param_1 = param_1 + 1;
  }
  return;
}

