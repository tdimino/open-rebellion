
void __thiscall FUN_0041c6c0(void *this,HGLOBAL param_1)

{
  int iVar1;
  undefined2 *puVar2;
  undefined4 uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  int *piVar7;
  int *piVar8;
  
  puVar2 = (undefined2 *)LockResource(param_1);
  if (puVar2 != (undefined2 *)0x0) {
    *(undefined2 *)((int)this + 0x1c) = *puVar2;
    *(undefined2 *)((int)this + 0x1e) = puVar2[1];
    *(undefined4 *)((int)this + 0x28) = *(undefined4 *)(puVar2 + 2);
    *(undefined1 *)((int)this + 0x2c) = *(undefined1 *)(puVar2 + 4);
    *(undefined2 *)((int)this + 0x20) = *(undefined2 *)((int)puVar2 + 9);
    *(undefined2 *)((int)this + 0x22) = *(undefined2 *)((int)puVar2 + 0xb);
    *(undefined2 *)((int)this + 0x24) = *(undefined2 *)((int)puVar2 + 0xd);
    piVar7 = (int *)((int)puVar2 + 0x11);
    *(undefined2 *)((int)this + 0x26) = *(undefined2 *)((int)puVar2 + 0xf);
    uVar3 = FUN_00618b70(*(uint *)((int)this + 0x28));
    *(undefined4 *)((int)this + 0x34) = uVar3;
    uVar3 = FUN_00618b70((int)*(short *)((int)this + 0x1e) << 2);
    iVar4 = 0;
    *(undefined4 *)((int)this + 0x30) = uVar3;
    if (0 < *(short *)((int)this + 0x1e)) {
      do {
        iVar1 = *piVar7;
        piVar7 = piVar7 + 1;
        *(int *)(*(int *)((int)this + 0x30) + iVar4 * 4) = iVar1 + *(int *)((int)this + 0x34);
        iVar4 = iVar4 + 1;
      } while (iVar4 < *(short *)((int)this + 0x1e));
    }
    uVar6 = *(uint *)((int)this + 0x28);
    piVar8 = *(int **)((int)this + 0x34);
    for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
      *piVar8 = *piVar7;
      piVar7 = piVar7 + 1;
      piVar8 = piVar8 + 1;
    }
    for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
      *(char *)piVar8 = (char)*piVar7;
      piVar7 = (int *)((int)piVar7 + 1);
      piVar8 = (int *)((int)piVar8 + 1);
    }
    FreeResource(param_1);
    FUN_0041c930((int)this);
  }
  return;
}

