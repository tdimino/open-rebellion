
void __thiscall FUN_00613a90(void *this,undefined4 *param_1)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined4 uStack_30;
  undefined4 *puStack_2c;
  undefined4 **ppuStack_28;
  undefined1 *puStack_24;
  undefined1 *puStack_20;
  uint uStack_1c;
  undefined4 local_c;
  undefined1 local_8 [4];
  undefined1 local_4 [4];
  
  puVar4 = param_1;
  if ((param_1 != (undefined4 *)0x0) && (piVar1 = *(int **)((int)this + 0x34), piVar1 != (int *)0x0)
     ) {
    puStack_20 = local_8;
    uStack_1c = 0;
    puStack_24 = local_4;
    ppuStack_28 = &param_1;
    puStack_2c = &local_c;
    uStack_30 = *(undefined4 *)((int)this + 0x60);
    iVar2 = (**(code **)(*piVar1 + 0x2c))(piVar1,0);
    if (iVar2 == -0x7787ff6a) {
      (**(code **)(**(int **)((int)this + 0x34) + 0x50))(*(int **)((int)this + 0x34));
      iVar2 = (**(code **)(**(int **)((int)this + 0x34) + 0x2c))
                        (*(int **)((int)this + 0x34),0,*(undefined4 *)((int)this + 0x60),&uStack_30,
                         &puStack_20,&ppuStack_28,&puStack_2c,0);
    }
    if (iVar2 == 0) {
      puVar5 = puStack_2c;
      for (uVar3 = uStack_1c >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        *puVar5 = *puVar4;
        puVar4 = puVar4 + 1;
        puVar5 = puVar5 + 1;
      }
      for (uVar3 = uStack_1c & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *(undefined1 *)puVar5 = *(undefined1 *)puVar4;
        puVar4 = (undefined4 *)((int)puVar4 + 1);
        puVar5 = (undefined4 *)((int)puVar5 + 1);
      }
      (**(code **)(**(int **)((int)this + 0x34) + 0x4c))
                (*(int **)((int)this + 0x34),puStack_2c,uStack_1c,puStack_24,ppuStack_28);
    }
  }
  return;
}

