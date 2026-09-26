
void __fastcall FUN_00458500(int param_1)

{
  undefined4 *puVar1;
  undefined *puVar2;
  int iVar3;
  undefined4 *puVar4;
  
  puVar4 = (undefined4 *)(param_1 + 0x1c0);
  iVar3 = 3;
  do {
    puVar1 = (undefined4 *)puVar4[-1];
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
    puVar2 = (undefined *)*puVar4;
    if (puVar2 != (undefined *)0x0) {
      FUN_00601990((int)puVar2);
      FUN_00618b60(puVar2);
    }
    puVar2 = (undefined *)puVar4[1];
    if (puVar2 != (undefined *)0x0) {
      FUN_00601990((int)puVar2);
      FUN_00618b60(puVar2);
    }
    puVar2 = (undefined *)puVar4[3];
    if (puVar2 != (undefined *)0x0) {
      FUN_00601990((int)puVar2);
      FUN_00618b60(puVar2);
    }
    puVar2 = (undefined *)puVar4[2];
    if (puVar2 != (undefined *)0x0) {
      FUN_00601990((int)puVar2);
      FUN_00618b60(puVar2);
    }
    if ((undefined4 *)puVar4[5] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)puVar4[5])(1);
    }
    puVar4 = puVar4 + 0xe;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  return;
}

