
void __thiscall FUN_005b64e0(void *this,uint param_1)

{
  uint uVar1;
  bool bVar2;
  uint uVar3;
  uint *puVar4;
  
  bVar2 = false;
  if (param_1 < 4) {
    uVar1 = *(uint *)((int)this + 0x130);
    uVar3 = 0;
    if (uVar1 != 0) {
      puVar4 = (uint *)((int)this + 0x34c);
      do {
        if (*puVar4 == param_1) {
          bVar2 = true;
          break;
        }
        uVar3 = uVar3 + 1;
        puVar4 = puVar4 + 1;
      } while (uVar3 < uVar1);
    }
    if (!bVar2) {
      *(uint *)((int)this + 0x130) = uVar1 + 1;
      *(uint *)((int)this + uVar3 * 4 + 0x34c) = param_1;
    }
  }
  return;
}

