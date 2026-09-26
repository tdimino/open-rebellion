
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005b6320(void *this,float param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  uint uVar4;
  float10 fVar5;
  float10 fVar6;
  
  iVar1 = (**(code **)(*(int *)this + 0x20))();
  if (iVar1 == 1) {
    fVar5 = FUN_005b1770(this,*(float *)((int)this + 0x3bc));
    *(float *)((int)this + 0x3bc) = (float)fVar5;
    if ((float10)*(float *)((int)this + 0x3c0) < fVar5) {
      fVar5 = (float10)*(float *)((int)this + 0x3c0);
    }
    *(float *)((int)this + 0x3c0) = (float)fVar5;
  }
  fVar5 = (float10)(**(code **)(*(int *)this + 100))();
  if (_DAT_0066c4f0 < *(float *)((int)this + 0x3bc) - *(float *)((int)this + 0x3c0)) {
    fVar5 = fVar5 * (float10)param_1 + (float10)*(float *)((int)this + 0x3c0);
    if ((float10)*(float *)((int)this + 0x3bc) <= fVar5) {
      fVar5 = (float10)*(float *)((int)this + 0x3bc);
    }
    *(float *)((int)this + 0x3c0) = (float)fVar5;
  }
  fVar5 = (float10)(**(code **)(*(int *)this + 0x60))();
  uVar4 = 1;
  fVar5 = fVar5 * (float10)param_1;
  if (*(int *)((int)this + 0x130) != 0) {
    piVar3 = (int *)((int)this + 0x34c);
    do {
      if (fVar5 <= (float10)_DAT_0066c4f0) break;
      iVar1 = *piVar3;
      fVar6 = (float10)*(float *)((int)this + iVar1 * 0x74 + 0x1e8) -
              (float10)*(float *)((int)this + iVar1 * 0x74 + 0x1e4);
      if (fVar5 < fVar6) {
        *(float *)((int)this + iVar1 * 0x74 + 0x1e4) =
             (float)(fVar5 + (float10)*(float *)((int)this + iVar1 * 0x74 + 0x1e4));
      }
      else {
        *(undefined4 *)((int)this + iVar1 * 0x74 + 0x1e4) =
             *(undefined4 *)((int)this + iVar1 * 0x74 + 0x1e8);
        fVar5 = fVar5 - fVar6;
        *(undefined4 *)((int)this + *piVar3 * 0x74 + 0x1e0) = 1;
      }
      uVar4 = uVar4 + 1;
      piVar3 = piVar3 + 1;
    } while (uVar4 <= *(uint *)((int)this + 0x130));
  }
  uVar4 = 0;
  if (*(int *)((int)this + 0x130) != 0) {
    piVar3 = (int *)((int)this + 0x34c);
    do {
      if (*(int *)((int)this + *piVar3 * 0x74 + 0x1e0) == 1) {
        if (uVar4 < 3) {
          iVar1 = 3 - uVar4;
          piVar2 = piVar3;
          do {
            *piVar2 = piVar2[1];
            piVar2 = piVar2 + 1;
            iVar1 = iVar1 + -1;
          } while (iVar1 != 0);
        }
        if (*(int *)((int)this + 0x130) == 4) {
          *(undefined4 *)((int)this + 0x358) = 0;
        }
        *(int *)((int)this + 0x130) = *(int *)((int)this + 0x130) + -1;
      }
      uVar4 = uVar4 + 1;
      piVar3 = piVar3 + 1;
    } while (uVar4 < *(uint *)((int)this + 0x130));
  }
  return;
}

