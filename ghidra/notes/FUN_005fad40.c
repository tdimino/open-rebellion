
int __thiscall FUN_005fad40(void *this,undefined4 *param_1,ushort param_2)

{
  uint uVar1;
  uint uVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  undefined4 local_200;
  ushort local_1fc;
  undefined1 local_1fa;
  undefined1 local_1f9;
  undefined1 local_1f8;
  undefined4 local_1f7 [125];
  
  if (3 < param_2) {
    local_1fc = *(ushort *)((int)this + 0x24);
    local_200 = *(undefined4 *)((int)this + 0x20);
    local_1f9 = *(undefined1 *)((int)this + 0x27);
    local_1f8 = *(undefined1 *)((int)this + 0x28);
    local_1fa = *(undefined1 *)((int)this + 0x26);
    uVar2 = local_1fc + 9;
    if (uVar2 <= param_2) {
      puVar3 = (undefined4 *)((int)this + 0x29);
      puVar4 = local_1f7;
      for (uVar1 = (uint)(local_1fc >> 2); uVar1 != 0; uVar1 = uVar1 - 1) {
        *puVar4 = *puVar3;
        puVar3 = puVar3 + 1;
        puVar4 = puVar4 + 1;
      }
      for (uVar1 = local_1fc & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
        *(undefined1 *)puVar4 = *(undefined1 *)puVar3;
        puVar3 = (undefined4 *)((int)puVar3 + 1);
        puVar4 = (undefined4 *)((int)puVar4 + 1);
      }
      puVar3 = &local_200;
      for (uVar1 = uVar2 >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
        *param_1 = *puVar3;
        puVar3 = puVar3 + 1;
        param_1 = param_1 + 1;
      }
      for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
        *(undefined1 *)param_1 = *(undefined1 *)puVar3;
        puVar3 = (undefined4 *)((int)puVar3 + 1);
        param_1 = (undefined4 *)((int)param_1 + 1);
      }
      return *(ushort *)((int)this + 0x24) + 9;
    }
  }
  return 0;
}

