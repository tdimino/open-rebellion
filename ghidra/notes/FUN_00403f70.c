
undefined4 __thiscall FUN_00403f70(void *this,ushort *param_1,uint param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  undefined4 *puVar5;
  
  puVar1 = (undefined4 *)((int)this + 0x18);
  if (*(int *)((int)this + 0x24) != 0) {
    FUN_00610c90((int)puVar1,1);
  }
  puVar5 = puVar1;
  for (iVar4 = 6; iVar4 != 0; iVar4 = iVar4 + -1) {
    *puVar5 = 0;
    puVar5 = puVar5 + 1;
  }
  uVar2 = *(uint *)((int)this + 0x28) & 0xfffffff9;
  uVar3 = uVar2 | 1;
  *(uint *)((int)this + 0x28) = uVar3;
  *(uint *)((int)this + 0x1c) = (uint)*param_1;
  *(uint *)((int)this + 0x28) =
       (param_2 & 1) << 3 | CONCAT31((int3)(uVar2 >> 8),(char)uVar3) & 0xfffffff7;
  *(uint *)((int)this + 0x20) = (uint)param_1[1];
  FUN_00610c30((int)puVar1);
  return *(undefined4 *)((int)this + 0x24);
}

