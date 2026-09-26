
void __thiscall FUN_0059b730(void *this,UINT param_1,uint param_2,uint param_3)

{
  int iVar1;
  uint local_20 [8];
  
  iVar1 = 0;
  if ((param_1 == 0x100) || (param_1 == 0x104)) {
    switch(param_2 & 0xff) {
    case 9:
      if ((*(int *)((int)this + 0x10c) != 0) && ((param_3 & 0x40000000) == 0)) {
        SendMessageA(*(HWND *)(*(int *)((int)this + 0x10c) + 0x18),param_1,param_2,0);
        return;
      }
      break;
    case 0xd:
    case 0x6e:
      FUN_0059ca90(this,0xad21,1);
      return;
    case 0x20:
      FUN_00595b10(0x20);
      break;
    case 0x21:
    case 0x69:
    case 0x6b:
      FUN_0059ca90(this,0xad1b,1);
      return;
    case 0x22:
    case 99:
    case 0x6d:
      FUN_0059ca90(this,0xad1c,1);
      return;
    case 0x23:
    case 0x61:
      FUN_0059ca90(this,0xad1a,1);
      return;
    case 0x24:
    case 0x67:
      FUN_0059ca90(this,0xad19,1);
      return;
    case 0x25:
    case 100:
      FUN_0059ca90(this,0xad1d,1);
      return;
    case 0x26:
    case 0x68:
      FUN_0059ca90(this,0xad1f,1);
      return;
    case 0x27:
    case 0x66:
      FUN_0059ca90(this,0xad1e,1);
      return;
    case 0x28:
    case 0x62:
      FUN_0059ca90(this,0xad20,1);
      return;
    case 0x31:
      FUN_0059b430((int)this);
      FUN_0059b600((int)this);
      FUN_0059ca90(this,0xad02,1);
      return;
    case 0x32:
      FUN_0059b430((int)this);
      FUN_0059b600((int)this);
      FUN_0059ca90(this,0xad03,1);
      return;
    case 0x33:
      FUN_0059b430((int)this);
      FUN_0059b600((int)this);
      FUN_0059ca90(this,0xad04,1);
      return;
    case 0x34:
      FUN_0059b430((int)this);
      FUN_0059b600((int)this);
      FUN_0059ca90(this,0xad05,1);
      return;
    case 0x35:
      FUN_0059b430((int)this);
      FUN_0059b600((int)this);
      FUN_0059ca90(this,0xad06,1);
      return;
    case 0x36:
      FUN_0059b430((int)this);
      FUN_0059b600((int)this);
      FUN_0059ca90(this,0xad07,1);
      return;
    case 0x37:
      FUN_0059b430((int)this);
      FUN_0059b600((int)this);
      FUN_0059ca90(this,0xad08,1);
      return;
    case 0x38:
      FUN_0059b430((int)this);
      FUN_0059b600((int)this);
      FUN_0059ca90(this,0xad09,1);
      return;
    case 0x77:
      iVar1 = 1;
    case 0x76:
      iVar1 = iVar1 + 1;
    case 0x75:
      iVar1 = iVar1 + 1;
    case 0x74:
      iVar1 = iVar1 + 1;
    case 0x73:
      iVar1 = iVar1 + 1;
    case 0x72:
      iVar1 = iVar1 + 1;
    case 0x71:
      iVar1 = iVar1 + 1;
    case 0x70:
      FUN_00595ba0(local_20);
      if (local_20[iVar1] != 0) {
        FUN_005c2990(*(void **)((int)this + 0x108),0xb,iVar1);
        return;
      }
      break;
    case 0x7b:
      iVar1 = 1;
    case 0x7a:
      iVar1 = iVar1 + 1;
    case 0x79:
      iVar1 = iVar1 + 1;
    case 0x78:
      FUN_00595bc0(local_20);
      if (local_20[iVar1] != 0) {
        FUN_005c2990(*(void **)((int)this + 0x108),0xb,iVar1 + 8);
        return;
      }
    }
  }
  else {
    switch(param_2) {
    case 9:
      if (*(int *)((int)this + 0x10c) != 0) {
        SendMessageA(*(HWND *)(*(int *)((int)this + 0x10c) + 0x18),param_1,param_2,0);
        return;
      }
      break;
    case 0xd:
    case 0x6e:
      FUN_0059ca90(this,0xad21,0);
      return;
    case 0x21:
    case 0x69:
    case 0x6b:
      FUN_0059ca90(this,0xad1b,0);
      return;
    case 0x22:
    case 99:
    case 0x6d:
      FUN_0059ca90(this,0xad1c,0);
      return;
    case 0x23:
    case 0x61:
      FUN_0059ca90(this,0xad1a,0);
      return;
    case 0x24:
    case 0x67:
      FUN_0059ca90(this,0xad19,0);
      return;
    case 0x25:
    case 100:
      FUN_0059ca90(this,0xad1d,0);
      return;
    case 0x26:
    case 0x68:
      FUN_0059ca90(this,0xad1f,0);
      return;
    case 0x27:
    case 0x66:
      FUN_0059ca90(this,0xad1e,0);
      return;
    case 0x28:
    case 0x62:
      FUN_0059ca90(this,0xad20,0);
      return;
    }
  }
  return;
}

