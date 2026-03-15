
int __thiscall FUN_005ce950(void *this,undefined4 param_1)

{
  int iVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  
  iVar1 = FUN_005ce890(this,param_1);
  if (iVar1 != 0) {
    iVar1 = FUN_005ee480(iVar1);
    return iVar1;
  }
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  puVar3 = (undefined4 *)0x0;
  if (puVar2 != (undefined4 *)0x0) {
    puVar2[1] = 0;
    puVar2[2] = 0;
    puVar2[3] = 0;
    puVar2[4] = 0;
    *(undefined2 *)(puVar2 + 5) = 0;
    *puVar2 = &PTR_FUN_0066d04c;
    puVar2[6] = param_1;
    puVar3 = puVar2;
  }
  FUN_005ceaa0(this,puVar3);
  iVar1 = FUN_005ee480((int)puVar3);
  FUN_005ceb70(this,param_1);
  return iVar1;
}

