
undefined4 FUN_005fba70(int *param_1)

{
  uint uVar1;
  HWND pHVar2;
  int iVar3;
  int iVar4;
  undefined4 *puVar5;
  
  if ((param_1[1] == 0x100) && (param_1[2] == 0xd)) {
    GetClassNameA((HWND)*param_1,&lpClassName_006be3c0,0xff);
    uVar1 = FUN_00626ad0(&lpClassName_006be3c0,&DAT_006ac798);
    if (uVar1 == 0) {
      pHVar2 = GetParent((HWND)*param_1);
      uVar1 = FUN_00600f60(pHVar2);
      iVar4 = 0;
      if (uVar1 != 0) {
        iVar4 = FUN_00604540((void *)(uVar1 + 0x6c),*param_1);
      }
      puVar5 = (undefined4 *)&lpClassName_006be3c0;
      for (iVar3 = 0x3f; iVar3 != 0; iVar3 = iVar3 + -1) {
        *puVar5 = 0;
        puVar5 = puVar5 + 1;
      }
      *(undefined2 *)puVar5 = 0;
      *(undefined1 *)((int)puVar5 + 2) = 0;
      param_1[1] = 0x407;
      *param_1 = (int)pHVar2;
      param_1[3] = iVar4;
      if (iVar4 != 0) {
        param_1[2] = *(int *)(iVar4 + 0x24);
        return 1;
      }
      param_1[2] = 0;
    }
  }
  return 1;
}

