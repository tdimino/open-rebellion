
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005b54d0(void *this,float param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  void *pvVar4;
  uint uVar5;
  bool bVar6;
  int iVar7;
  int iVar8;
  
  fVar1 = *(float *)((int)this + 0x3cc);
  fVar2 = *(float *)((int)this + 0x3c0);
  if (fVar1 != _DAT_0066c4f0) {
    fVar3 = *(float *)((int)this + 0x3c0) - param_1;
    bVar6 = _DAT_0066c4f0 < fVar2;
    *(float *)((int)this + 0x3c0) = fVar3;
    if (bVar6) {
      param_1 = (_DAT_0066c51c / fVar2) * (fVar2 - fVar3);
      if (param_1 < _DAT_0066c514) {
        param_1 = 1.0;
      }
      iVar8 = 100;
      iVar7 = 0;
      pvVar4 = (void *)FUN_00596f30();
      uVar5 = FUN_005af420(pvVar4,iVar7,iVar8);
      if ((float)uVar5 <= param_1) {
        (**(code **)(*(int *)this + 0xa0))(0);
      }
    }
    fVar2 = _DAT_0066c4f0;
    if (*(float *)((int)this + 0x3c0) < _DAT_0066c4f0) {
      fVar3 = _DAT_0066c500;
      if (*(float *)((int)this + 0x3c0) <= _DAT_0066c500) {
        fVar3 = *(float *)((int)this + 0x3c0);
      }
      fVar3 = fVar3 + *(float *)((int)this + 0x3cc);
      *(undefined4 *)((int)this + 0x3c0) = 0;
      *(float *)((int)this + 0x3cc) = fVar3;
      if (fVar3 < fVar2) {
        fVar3 = _DAT_0066c4f0;
      }
      *(float *)((int)this + 0x3cc) = fVar3;
      iVar8 = 100;
      iVar7 = 0;
      pvVar4 = (void *)FUN_00596f30();
      uVar5 = FUN_005af420(pvVar4,iVar7,iVar8);
      fVar1 = ((*(float *)((int)this + 0x3cc) - fVar1) * _DAT_0066c51c) / fVar1 + (float)uVar5;
      if (_DAT_0066c520 < fVar1) {
        if (fVar1 <= _DAT_0066c524) {
          (**(code **)(*(int *)this + 0xa0))(0);
          return;
        }
        if (fVar1 <= _DAT_0066c528) {
          (**(code **)(*(int *)this + 0xa0))(1);
          return;
        }
        if (fVar1 <= _DAT_0066c52c) {
          (**(code **)(*(int *)this + 0xa0))(2);
          return;
        }
        if (fVar1 <= _DAT_0066c530) {
          (**(code **)(*(int *)this + 0xa0))(3);
          return;
        }
        if (fVar1 <= _DAT_0066c51c) {
          (**(code **)(*(int *)this + 0xa0))(4);
        }
      }
    }
  }
  return;
}

